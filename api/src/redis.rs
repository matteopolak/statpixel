use std::sync::Arc;

use once_cell::sync::Lazy;
use redis::{aio::Connection, AsyncCommands, Client, RedisError};

use crate::{
	player::{data::Data, Player},
	Error,
};

static REDIS: Lazy<Client> = Lazy::new(|| {
	let url = dotenvy_macro::dotenv!("REDIS_URL");

	Client::open(url).expect("failed to connect to Redis")
});

async fn get_connection() -> Result<Connection, RedisError> {
	REDIS.get_async_connection().await
}

impl Player {
	/// # Errors
	/// Returns an error if the player's data could not be fetched
	pub async fn get_display_string_owned(self) -> Result<String, Arc<Error>> {
		self.get_display_string().await
	}

	/// # Errors
	/// Returns an error if the player's data could not be fetched
	pub async fn get_display_string(&self) -> Result<String, Arc<Error>> {
		let mut conn = get_connection()
			.await
			.map_err(|e| Arc::new(Error::Redis(e)))?;

		if let Ok(display) = conn.get(self.uuid.as_bytes()).await {
			return Ok(display);
		}

		let data = self.get_data().await?;
		let display = if let Some(display) = data.get_rank().as_coloured_str() {
			format!("{} {}", display, data.username)
		} else {
			format!("§7{}", data.username)
		};

		conn.set(self.uuid.as_bytes(), &display)
			.await
			.map_err(|e| Arc::new(Error::Redis(e)))?;

		Ok(display)
	}

	/// # Errors
	/// Returns an error if the Redis cache is unavailable
	pub async fn set_display_str(&self, data: &Data) -> Result<(), Error> {
		let mut conn = get_connection().await?;

		let display = if let Some(display) = data.get_rank().as_coloured_str() {
			format!("{} {}", display, data.username)
		} else {
			format!("§7{}", data.username)
		};

		conn.set(self.uuid.as_bytes(), &display).await?;

		Ok(())
	}
}
