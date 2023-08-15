use api::{player, player_old, snapshot::user::encode};
use database::{schema::snapshot, PostgresPool};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use flate2::read::ZlibDecoder;
use futures::StreamExt;
use tracing::info;
use uuid::Uuid;

pub fn decode_old(data: &[u8]) -> Result<player_old::data::Data, crate::Error> {
	let mut decoder = ZlibDecoder::new(data);

	Ok(bincode::decode_from_std_read(
		&mut decoder,
		bincode::config::standard(),
	)?)
}

/// Upgrades all old VERSION snapshots to new VERSION
pub async fn all(pool: PostgresPool) -> Result<PostgresPool, crate::Error> {
	loop {
		let snapshots = snapshot::table
			.filter(snapshot::version.eq(player_old::data::VERSION))
			.select((snapshot::data, snapshot::id, snapshot::uuid))
			.limit(1_000)
			.load::<(Vec<u8>, i32, Uuid)>(&mut pool.get().await?)
			.await?;

		let len = snapshots.len();

		futures::stream::iter(snapshots)
			.map(|(snapshot, id, uuid)| {
				let pool = &pool;

				async move {
					let mut data: player::data::Data =
						decode_old(snapshot.as_slice()).unwrap().into();

					// FIXME: Remove this after converting from v12 to v14
					data.uuid = uuid;

					let encoded = encode(&data).unwrap();
					let new_hash = fxhash::hash64(&encoded) as i64;

					diesel::update(snapshot::table.filter(snapshot::id.eq(id)))
						.set((
							snapshot::data.eq(encoded),
							snapshot::hash.eq(new_hash),
							snapshot::version.eq(player::VERSION),
						))
						.execute(&mut pool.get().await.unwrap())
						.await
						.unwrap();

					info!(id = id, "upgraded snapshot");

					Ok::<_, crate::Error>(())
				}
			})
			.buffer_unordered(20)
			.count()
			.await;

		if len < 1_000 {
			break;
		}
	}

	Ok(pool)
}
