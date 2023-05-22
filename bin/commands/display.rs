use database::schema;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use translate::tr;

use crate::{format::Display, util::success_embed, Context, Error};

/// Changes the way responses are displayed.
#[poise::command(slash_command, required_bot_permissions = "EMBED_LINKS")]
pub async fn display(ctx: Context<'_>, format: Display) -> Result<(), Error> {
	let u = ctx.author();

	diesel::insert_into(schema::user::table)
		.values((
			schema::user::display.eq(&format),
			schema::user::id.eq(u.id.0 as i64),
		))
		.on_conflict(schema::user::id)
		.do_update()
		.set(schema::user::display.eq(&format))
		.execute(&mut ctx.data().pool.get().await?)
		.await?;

	ctx.send(|m| {
		success_embed(
			m,
			tr!(ctx, "display-changed"),
			match format {
				Display::Image => tr!(ctx, "display-changed-image-description"),
				Display::Compact => tr!(ctx, "display-changed-compact-description"),
				Display::Text => tr!(ctx, "display-changed-text-description"),
			},
		)
	})
	.await?;

	Ok(())
}
