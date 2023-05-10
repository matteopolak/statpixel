pub mod arcade;
pub mod arena;
pub mod bedwars;
pub mod blitz;
pub mod build_battle;
pub mod cache;
pub mod display;
pub mod link;
pub mod skywars;
pub mod unlink;

#[macro_export]
macro_rules! generate_command {
	($game: ty, $mode: ty, $fn: ident) => {
		#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
		pub async fn $fn(
			ctx: $crate::Context<'_>,
			#[max_length = 16] username: Option<::std::string::String>,
			#[min_length = 32]
			#[max_length = 36]
			uuid: Option<::std::string::String>,
			mode: Option<$mode>,
		) -> ::std::result::Result<(), ::translate::Error> {
			let (_player, data, session) = $crate::get_data!(ctx, uuid, username);

			let png: ::std::borrow::Cow<[u8]> = {
				let mut surface = <$game>::canvas(ctx, &data, &session, mode);

				::api::canvas::to_png(&mut surface).into()
			};

			ctx.send(move |m| {
				m.attachment(::poise::serenity_prelude::AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".into(),
				})
			})
			.await?;

			Ok(())
		}
	};
}

/// Generates the code needed to fetch the player, their data, and their session.
#[macro_export]
macro_rules! get_data {
	($ctx: ident, $uuid: ident, $username: ident) => {{
		let player = match $crate::util::get_player_from_input(
			$ctx,
			$ctx.author(),
			$uuid,
			$username,
		)
		.await
		{
			Ok(player) => player,
			Err($crate::Error::NotLinked) => {
				$ctx.send(|m| {
					$crate::util::error_embed(
						m,
						::translate::tr!($ctx, "not-linked"),
						::translate::tr!($ctx, "not-linked-description"),
					)
				})
				.await?;

				return Ok(());
			}
			Err(e) => return Err(e),
		};

		let (data, session) =
			poise::futures_util::future::join(player.get_data(), player.get_session()).await;

		let data = data?;
		let session = session?;

		(player, data, session)
	}};
}
