use std::sync::Arc;

use axum::{
	extract::State,
	headers::{authorization::Credentials, Authorization},
	http::{HeaderValue, StatusCode},
	response::IntoResponse,
	Json, TypedHeader,
};
use database::schema::user;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::AppState;

pub struct Plain(pub String);

impl Credentials for Plain {
	const SCHEME: &'static str = "Plain";

	fn decode(value: &HeaderValue) -> Option<Self> {
		value.to_str().ok().map(|s| Self(s.to_owned()))
	}

	fn encode(&self) -> HeaderValue {
		self.0.parse().unwrap()
	}
}

#[derive(Deserialize)]
pub struct Vote {
	pub user: String,
	#[serde(rename = "isWeekend")]
	pub is_weekend: bool,
}

static SECRET: Lazy<String> = Lazy::new(|| std::env::var("TOPGG_SECRET").unwrap());

pub async fn add_vote(
	State(state): State<Arc<AppState>>,
	TypedHeader(token): TypedHeader<Authorization<Plain>>,
	Json(vote): Json<Vote>,
) -> Result<impl IntoResponse, StatusCode> {
	if token.0 .0 != SECRET.as_str() {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let Ok(id) = vote.user.parse::<u64>() else {
		return Err(StatusCode::BAD_REQUEST);
	};

	diesel::insert_into(user::table)
		.values((&user::id.eq(id as i64), &user::votes.eq(1)))
		.on_conflict(user::id)
		.do_update()
		.set(user::votes.eq(user::votes + 1))
		.execute(
			&mut state
				.pool
				.get()
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
		)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(())
}
