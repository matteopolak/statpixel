use api::{
	canvas::{self, body::Body, shape, text, Canvas},
	player::Player,
};
use futures::StreamExt;
use minecraft::{paint::Paint, text::Text};
use poise::serenity_prelude::{AttachmentType, CreateEmbed};
use skia_safe::textlayout::TextAlign;
use translate::{Context, Error};

use crate::{format::Display, util::escape_username};

async fn autocomplete_board(
	_ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	let Ok(leaderboards) = api::leaderboard::get().await else {
		return Box::new(std::iter::empty());
	};

	let lower = partial.to_ascii_lowercase();

	Box::new(
		leaderboards
			.into_iter()
			.filter_map(|board| {
				if !board.display_name.to_ascii_lowercase().starts_with(&lower) {
					return None;
				}

				Some(board.display_name)
			})
			.take(10)
			.collect::<Vec<_>>()
			.into_iter(),
	)
}

#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn leaderboard(
	ctx: Context<'_>,
	#[autocomplete = "autocomplete_board"] board: String,
) -> Result<(), Error> {
	ctx.defer().await?;

	let format = crate::util::get_format_from_input(ctx).await;
	let leaderboard = {
		let leaderboards = api::leaderboard::get().await?;
		let Some(leaderboard) = leaderboards.into_iter().find(|l| l.display_name == board) else {
			return Err(Error::Custom(format!("No leaderboard found with the name `{}`.", board)));
		};

		leaderboard
	};

	let api::leaderboard::Leaderboard {
		name,
		leaders,
		game,
		path,
		display_name,
	} = leaderboard;

	let leaders = futures::stream::iter(
		leaders
			.into_iter()
			.map(Player::from_uuid_unchecked)
			.map(Player::get_data_owned),
	)
	.buffered(10)
	.filter_map(|r| async { r.ok() })
	.collect::<Vec<_>>()
	.await;

	let leaderboard = api::leaderboard::Leaderboard {
		name,
		leaders: vec![],
		game,
		path,
		display_name,
	};

	match format {
		Display::Image | Display::Compact => {
			let png = {
				let mut canvas = Canvas::new(720.).gap(7.).push_down(
					&shape::LeaderboardTitle,
					Body::new(24., TextAlign::Center)
						.extend(leaderboard.game.as_text())
						.extend(&[
							Text {
								text: " (",
								paint: Paint::White,
								..Default::default()
							},
							Text {
								text: &leaderboard.name,
								paint: Paint::White,
								..Default::default()
							},
							Text {
								text: ")",
								paint: Paint::White,
								..Default::default()
							},
						])
						.build(),
				);

				for (idx, player) in leaders.iter().enumerate() {
					let value = player
						.stats
						.get_value(&leaderboard.game, leaderboard.path.as_str())
						.unwrap_or_else(|| Box::new(0));

					canvas = canvas
						.push_down_start(
							&shape::LeaderboardPlace,
							shape::LeaderboardPlace::from_usize(idx + 1),
						)
						.push_right(
							&shape::LeaderboardName,
							Body::build_slice(
								&text::from_data(player, &player.username),
								20.,
								None,
							),
						)
						.push_right(
							&shape::LeaderboardValue,
							shape::LeaderboardValue::from_value(ctx, &value),
						);
				}

				let mut surface = canvas.build(None).unwrap();

				canvas::to_png(&mut surface).into()
			};

			ctx.send(move |m| {
				m.attachment(AttachmentType::Bytes {
					data: png,
					filename: "canvas.png".into(),
				})
			})
			.await?;
		}
		Display::Text => {
			let mut embed = CreateEmbed::default();

			embed.colour(crate::EMBED_COLOUR);
			embed.title(format!("{}", leaderboard.display_name));
			embed.description(format!(
				"{}",
				leaders
					.into_iter()
					.enumerate()
					.map(|(i, player)| {
						let value = player
							.stats
							.get_value(&leaderboard.game, leaderboard.path.as_str())
							.unwrap_or_else(|| Box::new(0));

						let value = value.to_formatted_label(ctx);

						if let Some(prefix) = player.get_rank().as_str() {
							format!(
								"{}. **{} {}** ({})",
								i + 1,
								prefix,
								escape_username(&player.username),
								value
							)
						} else {
							format!(
								"{}. **{}** ({})",
								i + 1,
								escape_username(&player.username),
								value
							)
						}
					})
					.intersperse("\n".to_string())
					.collect::<String>()
			));

			ctx.send(|m| {
				m.embeds.push(embed);
				m
			})
			.await?;
		}
	}

	Ok(())
}
