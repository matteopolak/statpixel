use crate::{format::Display, Context, Error};
use api::canvas::{
	self,
	builder::{body::Body, shape, text, Canvas},
	label::ToFormatted,
};
use minecraft::{
	calc::network,
	paint::Paint,
	text::{parse::minecraft_text, Text},
};
use poise::serenity_prelude::{AttachmentType, CreateEmbed};
use translate::tr;

const LABEL: [Text; 1] = minecraft_text("§f§lNetwork");

#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, required_bot_permissions = "ATTACH_FILES")]
pub async fn network(
	ctx: Context<'_>,
	#[max_length = 16]
	#[autocomplete = "crate::commands::autocomplete_username"]
	username: Option<String>,
	#[min_length = 32]
	#[max_length = 36]
	uuid: Option<String>,
) -> Result<(), Error> {
	ctx.defer().await?;

	let format = crate::util::get_format_from_input(ctx).await;

	match format {
		Display::Image | Display::Compact => {
			let (player, data, session, skin) = crate::get_all!(ctx, uuid, username);

			player.increase_searches(ctx).await?;

			let png = {
				let status = shape::Status(&session, skin.as_ref());
				let level = network::get_level(data.xp);
				let progress = shape::WideBubbleProgress(
					network::get_level_progress(data.xp),
					network::get_colours(level),
				);

				let mut surface = Canvas::new(720.)
					.gap(7.)
					.push_down(
						&shape::Title,
						shape::Title::from_text(&text::from_data(&data, &data.username)),
					)
					.push_down(
						&shape::Subtitle,
						shape::Subtitle::from_label(ctx, &LABEL, "member-profile"),
					)
					.push_down(
						&progress,
						shape::WideBubbleProgress::from_level_progress(
							ctx,
							&network::get_level_format(level),
							&network::get_curr_level_xp(data.xp),
							&network::get_level_xp(data.xp),
						),
					)
					.push_right_start(
						&canvas::builder::shape::Sidebar,
						canvas::builder::body::Body::default()
							.append_item(
								&::translate::tr!(ctx, "experience"),
								&data.xp.to_formatted_label(ctx),
								&Paint::Yellow,
							)
							.append_item(
								&::translate::tr!(ctx, "karma"),
								&data.karma.to_formatted_label(ctx),
								&Paint::LightPurple,
							)
							.append_item(
								&::translate::tr!(ctx, "rewards"),
								&data.rewards.to_formatted_label(ctx),
								&Paint::Gold,
							)
							.append_item(
								&::translate::tr!(ctx, "friend-requests"),
								&data.friend_requests.to_formatted_label(ctx),
								&Paint::Green,
							)
							.append_item(
								&::translate::tr!(ctx, "time-played"),
								&data.playtime.to_formatted_label(ctx),
								&Paint::Gold,
							)
							.append_item(
								&::translate::tr!(ctx, "first-login"),
								&data.first_login.to_formatted_label(ctx),
								&Paint::Aqua,
							)
							.append_item(
								&::translate::tr!(ctx, "last-login"),
								&data.last_login.to_formatted_label(ctx),
								&Paint::Blue,
							)
							.build(17., ::std::option::Option::None),
					)
					.push_right(&status, Body::from_status(ctx, &session))
					.build(None)
					.unwrap();

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
			let (player, data) = crate::get_data!(ctx, uuid, username);

			player.increase_searches(ctx).await?;

			let mut embed = CreateEmbed::default();

			embed.thumbnail(player.get_body_url());

			if let Some(prefix) = data.get_rank().as_str() {
				embed.author(|a| {
					a.name(format!("{} {} :: Network", prefix, player.username))
						.icon_url(player.get_head_url())
				});
			} else {
				embed.author(|a| {
					a.name(format!("{} :: Network", player.username))
						.icon_url(player.get_head_url())
				});
			}

			embed.description(format!(
				"{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**\n{}: **{}**",
				tr!(ctx, "experience"),
				data.xp.to_formatted_label(ctx),
				tr!(ctx, "karma"),
				data.karma.to_formatted_label(ctx),
				tr!(ctx, "rewards"),
				data.rewards.to_formatted_label(ctx),
				tr!(ctx, "friend-requests"),
				data.friend_requests.to_formatted_label(ctx),
				tr!(ctx, "time-played"),
				data.playtime.to_formatted_label(ctx),
				tr!(ctx, "first-login"),
				data.first_login.to_formatted_label(ctx),
				tr!(ctx, "last-login"),
				data.last_login.to_formatted_label(ctx),
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
