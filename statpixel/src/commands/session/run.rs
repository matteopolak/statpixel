use std::borrow::Cow;

use api::{command::Id, id, Player};
use chrono::{DateTime, Utc};
use database::schema::session;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use poise::serenity_prelude::{CreateActionRow, CreateButton, CreateEmbed, ReactionType};
use translate::{context, tr, tr_fmt};

use crate::Error;

pub async fn list(
	ctx: &context::Context<'_>,
	player: Option<Player>,
	page: u32,
) -> Result<(), Error> {
	let sessions: Vec<(uuid::Uuid, uuid::Uuid, DateTime<Utc>)> = if let Some(ref player) = player {
		session::table
			.select((session::id, session::uuid, session::created_at))
			.filter(session::uuid.eq(player.uuid))
			.order(session::created_at.desc())
			.limit(10)
			.offset(i64::from(page) * 10)
			.get_results(&mut ctx.data().pool.get().await?)
			.await?
	} else {
		session::table
			.select((session::id, session::uuid, session::created_at))
			.filter(session::user_id.eq(ctx.author().unwrap().id.0.get() as i64))
			.order(session::created_at.desc())
			.limit(10)
			.offset(i64::from(page) * 10)
			.get_results(&mut ctx.data().pool.get().await?)
			.await?
	};

	let embed = CreateEmbed::new()
		.title(tr_fmt!(ctx, "session-list-title", page: page + 1))
		.colour(crate::EMBED_COLOUR)
		.description(if sessions.is_empty() {
			tr(ctx, "session-list-empty")
		} else {
			Cow::Owned(
				sessions
					.into_iter()
					.map(|(id, uuid, created_at)| {
						format!(
							"- [**`{}`**](https://namemc.com/profile/{}) - <t:{}:R>",
							id,
							uuid,
							created_at.timestamp()
						)
					})
					.intersperse("\n".to_string())
					.collect::<String>(),
			)
		});

	let mut components = vec![];

	if page > 0 {
		components.push(
			CreateButton::new(id::command(Id::SessionPage {
				uuid: player.as_ref().map(|p| p.uuid),
				page: page - 1,
			}))
			.emoji(ReactionType::Unicode("⬅️".to_string())),
		);
	}

	components.push(
		CreateButton::new(id::command(Id::SessionPage {
			uuid: player.as_ref().map(|p| p.uuid),
			page: page + 1,
		}))
		.emoji(ReactionType::Unicode("➡️".to_string())),
	);

	ctx.send(
		poise::CreateReply::new()
			.embed(embed)
			.components(vec![CreateActionRow::Buttons(components)]),
	)
	.await?;

	Ok(())
}