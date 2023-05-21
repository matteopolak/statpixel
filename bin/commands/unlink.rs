use database::schema;
use diesel::{ExpressionMethods, RunQueryDsl};
use translate::tr;
use uuid::Uuid;

use crate::{
	util::{error_embed, success_embed},
	Context, Error,
};

/// Unlinks your Discord account from a Minecraft account.
#[poise::command(slash_command)]
pub async fn unlink(ctx: Context<'_>) -> Result<(), Error> {
	let removed = diesel::update(schema::user::table)
		.set(schema::user::uuid.eq::<Option<Uuid>>(None))
		.filter(schema::user::id.eq(ctx.author().id.0 as i64))
		.filter(schema::user::uuid.is_not_null())
		.execute(&mut ctx.data().pool.get()?)?;

	if removed > 0 {
		ctx.send(|m| {
			success_embed(
				m,
				tr!(ctx, "unlinking-succeeded"),
				tr!(ctx, "unlinking-succeeded-description"),
			)
		})
		.await?;
	} else {
		ctx.send(|m| {
			error_embed(
				m,
				tr!(ctx, "unlinking-failed"),
				tr!(ctx, "unlinking-failed-description"),
			)
		})
		.await?;
	}

	Ok(())
}
