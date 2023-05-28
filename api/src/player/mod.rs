pub mod data;
pub mod stats;
pub mod status;

use database::schema::autocomplete;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
use reqwest::{Request, StatusCode, Url};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use tracing::error;
use translate::Context;
use uuid::Uuid;

use crate::{
	cache::{PLAYER_CACHE, PLAYER_DATA_CACHE, PLAYER_SESSION_CACHE},
	http::HTTP,
	Error,
};

use self::status::Status;

pub const VERSION: i16 = 2;

static HYPIXEL_PLAYER_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/player").unwrap());

static HYPIXEL_STATUS_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.hypixel.net/status").unwrap());

static MOJANG_USERNAME_TO_UUID_API_ENDPOINT: Lazy<Url> =
	Lazy::new(|| Url::from_str("https://api.mojang.com/users/profiles/minecraft/").unwrap());

static MOJANG_UUID_TO_USERNAME_API_ENDPOINT: Lazy<Url> = Lazy::new(|| {
	Url::from_str("https://sessionserver.mojang.com/session/minecraft/profile/").unwrap()
});

#[derive(Deserialize, Debug)]
pub struct Response {
	#[serde(default)]
	pub player: data::Data,
	pub success: bool,
}

#[derive(Deserialize, Clone)]
pub struct MojangResponse {
	pub name: String,
	pub id: Uuid,
}

#[derive(Clone, Debug)]
pub struct Player {
	pub uuid: Uuid,
	pub username: String,
}

impl Player {
	#[must_use]
	pub fn new(uuid: Uuid, username: String) -> Self {
		Self { uuid, username }
	}

	/// Creates a new player from a uuid without any validation.
	/// The username will not be set propertly.
	#[must_use]
	pub fn from_uuid_unchecked(uuid: Uuid) -> Self {
		Self {
			uuid,
			username: String::new(),
		}
	}

	#[must_use]
	pub fn get_head_url(&self) -> String {
		format!("https://visage.surgeplay.com/head/64/{}?y=72.5", self.uuid)
	}

	#[must_use]
	pub fn get_body_url(&self) -> String {
		format!("https://visage.surgeplay.com/full/{}?y=20", self.uuid)
	}

	/// # Errors
	/// Returns an error if there is an issue with the database.
	pub async fn increase_searches(&self, ctx: Context<'_>) -> Result<(), translate::Error> {
		diesel::insert_into(autocomplete::table)
			.values((
				autocomplete::name.eq(&self.username),
				autocomplete::uuid.eq(&self.uuid),
				autocomplete::searches.eq(1),
			))
			.on_conflict(autocomplete::uuid)
			.do_update()
			.set((
				autocomplete::name.eq(&self.username),
				autocomplete::searches.eq(autocomplete::searches + 1),
			))
			.execute(&mut ctx.data().pool.get().await?)
			.await?;

		Ok(())
	}

	/// # Errors
	/// Returns an error if the username does not exist or if their data is invalid.
	pub async fn from_username(username: &str) -> Result<Player, Arc<Error>> {
		let player = PLAYER_CACHE
			.try_get_with(
				username.to_ascii_lowercase(),
				Self::from_username_raw(username),
			)
			.await?;

		Ok(player)
	}

	async fn from_username_raw(username: &str) -> Result<Player, Error> {
		let url = MOJANG_USERNAME_TO_UUID_API_ENDPOINT.join(username).unwrap();

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_mojang(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::UsernameNotFound(username.to_string()));
		}

		let response = response.json::<MojangResponse>().await?;
		let player = Self::new(response.id, response.name);

		// Also add the player with their uuid to the cache
		PLAYER_CACHE
			.insert(response.id.to_string(), player.clone())
			.await;

		Ok(player)
	}

	/// # Errors
	/// Returns an error if the uuid does not exist or if their data is invalid.
	pub async fn from_uuid(uuid: &Uuid) -> Result<Player, Arc<Error>> {
		PLAYER_CACHE
			.try_get_with(uuid.to_string(), Self::from_uuid_raw(uuid))
			.await
	}

	async fn from_uuid_raw(uuid: &Uuid) -> Result<Player, Error> {
		let url = MOJANG_UUID_TO_USERNAME_API_ENDPOINT
			.join(&uuid.to_string())
			.unwrap();

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_mojang(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::UuidNotFound(*uuid));
		}

		let response = response.json::<MojangResponse>().await?;
		let lower = response.name.to_ascii_lowercase();
		let player = Self::new(response.id, response.name);

		// Also add the player to the cache with the lower-case username
		PLAYER_CACHE.insert(lower, player.clone()).await;

		Ok(player)
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_data(&self) -> Result<data::Data, Arc<Error>> {
		Box::pin(PLAYER_DATA_CACHE.try_get_with_by_ref(&self.uuid, self.get_data_raw())).await
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_data_owned(self) -> Result<data::Data, Arc<Error>> {
		Box::pin(PLAYER_DATA_CACHE.try_get_with(self.uuid, self.get_data_raw())).await
	}

	async fn get_data_raw(&self) -> Result<data::Data, Error> {
		let url = {
			let mut url = HYPIXEL_PLAYER_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_hypixel(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::PlayerNotFound(self.username.clone()));
		}

		let response = match response.json::<Response>().await {
			Ok(response) => response,
			Err(err) => {
				error!("Failed to deserialize {} data: {}", self.uuid, err);

				return Err(err.into());
			}
		};

		Ok(response.player)
	}

	/// # Errors
	/// Returns an error if the player does not have a profile or if their data is invalid.
	pub async fn get_session(&self) -> Result<status::Session, Arc<Error>> {
		PLAYER_SESSION_CACHE
			.try_get_with_by_ref(&self.uuid, self.get_session_raw())
			.await
	}

	async fn get_session_raw(&self) -> Result<status::Session, Error> {
		let url = {
			let mut url = HYPIXEL_STATUS_API_ENDPOINT.clone();

			url.set_query(Some(&format!("uuid={}", self.uuid)));
			url
		};

		let req = Request::new(reqwest::Method::GET, url);
		let response = HTTP.perform_hypixel(req.into()).await?;

		if response.status() != StatusCode::OK {
			return Err(Error::SessionNotFound(self.username.clone()));
		}

		let response = response.json::<Status>().await?;

		Ok(response.session)
	}
}

#[cfg(test)]
mod tests {
	use std::assert_matches::assert_matches;

	use super::*;

	#[test]
	fn test_player() {
		let uuid = Uuid::new_v4();
		let player = Player::new(uuid, "Notch".to_string());

		assert_eq!(player.uuid, uuid);
		assert_eq!(player.username, "Notch".to_string());
	}

	#[tokio::test]
	async fn test_player_from_username() {
		let player = Player::from_username("Notch").await;

		assert_matches!(player, Ok(_));
		assert_eq!(
			player.unwrap().uuid,
			Uuid::parse_str("069a79f4-44e9-4726-a5be-fca90e38aaf5").unwrap(),
		);
	}

	#[tokio::test]
	async fn test_player_from_uuid() {
		let uuid = Uuid::parse_str("069a79f4-44e9-4726-a5be-fca90e38aaf5").unwrap();
		let player = Player::from_uuid(&uuid).await;

		assert_matches!(player, Ok(_));
		assert_eq!(player.unwrap().username, "Notch".to_string());
	}

	#[tokio::test]
	async fn test_player_data() {
		let uuid = Uuid::parse_str("b876ec32-e396-476b-a115-8438d83c67d4").unwrap();
		let player = Player::new(uuid, "Technoblade".to_string());

		assert_matches!(player.get_data().await, Ok(_));
	}
}
