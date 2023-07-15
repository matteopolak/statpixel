use chrono::{DateTime, Utc};
use diesel::prelude::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
	pub id: i64,
	pub uuid: Option<Uuid>,
	pub updated_at: DateTime<Utc>,
	pub created_at: DateTime<Utc>,
	pub display: i16,
	pub suffix: Option<String>,
	pub colour: Option<i32>,
	pub votes: i16,
}

#[derive(Queryable)]
pub struct Snapshot {
	pub id: i64,
	pub uuid: Uuid,
	pub data: Vec<u8>,

	pub created_at: chrono::DateTime<chrono::Utc>,
	pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Queryable)]
pub struct Metric {
	pub id: i64,
	pub kind: MetricKind,
	pub created_at: chrono::DateTime<chrono::Utc>,
}

pub enum MetricKind {
	GuildJoin,
	GuildLeave,
	ProfileCreate,
	CommandRun,
}

impl From<i16> for MetricKind {
	fn from(kind: i16) -> Self {
		match kind {
			0 => Self::GuildJoin,
			1 => Self::GuildLeave,
			2 => Self::ProfileCreate,
			3 => Self::CommandRun,
			_ => unreachable!(),
		}
	}
}

impl From<MetricKind> for i16 {
	fn from(kind: MetricKind) -> Self {
		match kind {
			MetricKind::GuildJoin => 0,
			MetricKind::GuildLeave => 1,
			MetricKind::ProfileCreate => 2,
			MetricKind::CommandRun => 3,
		}
	}
}
