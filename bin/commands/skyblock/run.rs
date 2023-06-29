use std::{borrow::Cow, cmp::max};

use api::{
	canvas::{self, body::Body, chart, label::ToFormatted, Canvas},
	id::SkyBlockMode,
	nbt::inventory::Item,
	player::Player,
	skyblock::{
		self,
		materials::MATERIALS,
		networth::{self, calc::Category},
		profile::TransactionAction,
	},
};
use canvas::{shape, text};
use chrono::Utc;
use minecraft::{
	calc::{network, sky_block},
	paint::Paint,
	style::MinecraftFont,
	text::{
		parse::{minecraft_string, minecraft_text},
		Text,
	},
};
use poise::serenity_prelude::CreateAttachment;
use skia_safe::textlayout::TextAlign;
use translate::{context, tr, Error};
use uuid::Uuid;

const LABEL: [Text; 2] = minecraft_text("§b§lSky§a§lBlock");
const EMPTY_ROW: &[Option<Item>] = &[None, None, None, None, None, None, None, None, None];

pub async fn auctions(
	ctx: &context::Context<'_>,
	username: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let png: Cow<_> = {
		let auctions = player.get_auctions().await?;
		let status = shape::Status(&session, skin.image());
		let level = network::get_level(data.xp);
		let progress = shape::WideBubbleProgress(
			network::get_level_progress(data.xp),
			network::get_colours(level),
		);

		let mut canvas = Canvas::new(720.)
			.gap(7.)
			.push_down(
				&shape::Title,
				shape::Title::from_text(&text::from_data(&data, &data.username, suffix.as_deref())),
			)
			.push_down(
				&shape::Subtitle,
				shape::Subtitle::from_label(ctx, &LABEL, "player-auctions"),
			)
			.push_down_post_draw(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&network::get_level_format(level),
					&network::get_curr_level_xp(data.xp),
					&network::get_level_xp(data.xp),
				),
			)
			.push_right_start_post_draw(&status, Body::from_status(ctx, &session));

		for auction in auctions.iter() {
			let mut text = minecraft_string(&auction.item.name).collect::<Vec<_>>();
			let bid = max(auction.starting_bid, auction.highest_bid);
			let bid = bid.to_formatted_label(ctx);

			text.extend([
				Text {
					text: "\n",
					size: None,
					..Default::default()
				},
				Text {
					text: bid.as_ref(),
					paint: Paint::Gold,
					font: MinecraftFont::Normal,
					size: Some(30.),
				},
			]);

			canvas = canvas.push_checked(
				&shape::TallBubble,
				Body::build_slice(text.as_slice(), 23., TextAlign::Center),
			);
		}

		canvas::to_png(&mut canvas.build(None, background).unwrap()).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.components(vec![SkyBlockMode::as_root(
				ctx,
				player.uuid,
				None,
				Some(SkyBlockMode::Auctions),
			)])
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

#[allow(clippy::too_many_lines)]
pub async fn profile(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let Some(profile) = (match profile {
		Some(profile) => data.stats.sky_block.profiles.iter().find(|p| p.name == profile),
		None => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
	};

	let name = profile.name.as_str();
	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let Some(member) = profile.members.get(&player.uuid) else {
		return Err(Error::MemberPlayerNotFound(data.username.clone()));
	};

	let png: Cow<_> = {
		let status = shape::Status(&session, skin.image());
		let level = sky_block::get_level(member.leveling.xp);
		let progress = shape::WideBubbleProgress(
			sky_block::get_level_progress(member.leveling.xp),
			sky_block::get_colours(level),
		);

		let ctx = &ctx;
		let mut surface = Canvas::new(720.)
			.gap(7.)
			.push_down(
				&shape::Title,
				shape::Title::from_text(&text::from_data(&data, &data.username, suffix.as_deref())),
			)
			.push_down(
				&shape::Subtitle,
				shape::Subtitle::from_label_str(&LABEL, name),
			)
			.push_down_post_draw(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&sky_block::get_level_format(level),
					&sky_block::get_curr_level_xp(member.leveling.xp),
					&sky_block::get_level_xp(member.leveling.xp),
				),
			)
			.push_right_start(
				&canvas::shape::Sidebar,
				canvas::body::Body::new(17., None)
					.append_item(
						&::translate::tr!(ctx, "coins"),
						&canvas::label::ToFormatted::to_formatted_label(&member.coin_purse, ctx),
						&Paint::Gold,
					)
					.append_item(
						&::translate::tr!(ctx, "fairy-souls"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fairy_souls_collected,
							ctx,
						),
						&Paint::Aqua,
					)
					.append_item(
						&::translate::tr!(ctx, "fairy-exchanges"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fairy_exchanges,
							ctx,
						),
						&Paint::LightPurple,
					)
					.append_item(
						&::translate::tr!(ctx, "fishing-treasure"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fishing_treasure_caught,
							ctx,
						),
						&Paint::Blue,
					)
					.append_item(
						&::translate::tr!(ctx, "zones-visited"),
						&canvas::label::ToFormatted::to_formatted_label(&member.zones_visited, ctx),
						&Paint::Green,
					)
					.append_item(
						&::translate::tr!(ctx, "generators-crafted"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.generators_crafted,
							ctx,
						),
						&Paint::White,
					)
					.append_item(
						&::translate::tr!(ctx, "highest-crit"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.stats.highest_critical_damage,
							ctx,
						),
						&Paint::Red,
					)
					.build(),
			)
			.push_right_post_draw(&status, Body::from_status(ctx, &session))
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.farming),
					tr!(ctx, "farming").as_ref(),
					Paint::Gold,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.mining),
					tr!(ctx, "mining").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.combat),
					tr!(ctx, "combat").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.foraging),
					tr!(ctx, "foraging").as_ref(),
					Paint::Green,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.fishing),
					tr!(ctx, "fishing-skill").as_ref(),
					Paint::White,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.enchanting),
					tr!(ctx, "enchanting").as_ref(),
					Paint::DarkPurple,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.alchemy),
					tr!(ctx, "alchemy").as_ref(),
					Paint::Yellow,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.taming),
					tr!(ctx, "taming").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_dungeoneering(
						member.dungeons.types.catacombs.experience,
					),
					tr!(ctx, "dungeoneering").as_ref(),
					Paint::Gray,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.carpentry),
					tr!(ctx, "carpentry").as_ref(),
					Paint::Red,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.runecrafting),
					tr!(ctx, "runecrafting").as_ref(),
					Paint::LightPurple,
				),
			)
			.push_checked(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&sky_block::skills::get_level_general(member.skills.social),
					tr!(ctx, "social").as_ref(),
					Paint::Green,
				),
			)
			.build(None, background)
			.unwrap();

		canvas::to_png(&mut surface).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.components(vec![SkyBlockMode::as_root(
				ctx,
				player.uuid,
				Some(name.to_string()),
				Some(SkyBlockMode::Profile(None)),
			)])
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

pub async fn bank(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (player, data) = crate::commands::get_player_data(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let Some(profile) = (match profile {
		Some(profile) => data.stats.sky_block.profiles.iter().find(|p| p.name == profile),
		None => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::MemberPlayerNotFound(data.username.clone()));
	};

	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let mut bank = skyblock::profile::Banking::clone(&profile.banking);

	let (lower, upper) = if bank.transactions.is_empty() {
		(0, 100)
	} else {
		let mut lower = u64::MAX;
		// The upper bound should be at least 100
		let mut upper = 100u64;

		// overwrite the bank transactions and replace the "change" by the total at that time
		for transaction in bank.transactions.iter_mut().rev() {
			match transaction.action {
				TransactionAction::Withdraw => bank.balance += transaction.amount,
				TransactionAction::Deposit => bank.balance -= transaction.amount,
			}

			transaction.amount = bank.balance;

			if transaction.amount < lower {
				lower = transaction.amount;
			}

			if transaction.amount > upper {
				upper = transaction.amount;
			}
		}

		(lower, upper)
	};

	let first = bank
		.transactions
		.first()
		.map_or_else(Utc::now, |t| t.timestamp);

	let last = bank
		.transactions
		.last()
		.map_or_else(Utc::now, |t| t.timestamp);

	let png: Cow<_> = {
		let mut buffer = chart::u64::create(
			ctx,
			vec![(
				tr!(ctx, "bank-balance"),
				bank.transactions
					.iter()
					.map(|t| (t.timestamp, t.amount))
					.collect(),
			)],
			first..last,
			(lower * 7 / 8)..(upper * 8 / 7),
			Some(Paint::Gold),
			background,
		)?;

		let mut surface = chart::canvas(&mut buffer)?;

		chart::apply_title(
			ctx,
			&mut surface,
			&data,
			&[Text {
				text: tr!(&ctx, "island-bank-balance").as_ref(),
				paint: Paint::Gold,
				..Default::default()
			}],
			background,
		);
		chart::round_corners(&mut surface);

		canvas::to_png(&mut surface).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.components(vec![SkyBlockMode::as_root(
				ctx,
				player.uuid,
				None,
				Some(SkyBlockMode::Bank(None)),
			)])
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

#[allow(clippy::too_many_lines)]
pub async fn networth(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let Some(profile) = (match profile {
		Some(profile) => data.stats.sky_block.profiles.iter().find(|p| p.name == profile),
		None => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
	};

	let name = profile.name.as_str();
	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let Some(member) = profile.members.get(&player.uuid) else {
		return Err(Error::MemberPlayerNotFound(data.username.clone()));
	};

	let prices = networth::prices().await?;
	let networth = member.networth(&prices);

	#[allow(clippy::cast_precision_loss)]
	let purse = member.coin_purse as f64;
	#[allow(clippy::cast_precision_loss)]
	let bank = profile.banking.balance as f64;

	let png: Cow<_> = {
		let status = shape::Status(&session, skin.image());
		let level = sky_block::get_level(member.leveling.xp);
		let progress = shape::WideBubbleProgress(
			sky_block::get_level_progress(member.leveling.xp),
			sky_block::get_colours(level),
		);

		let mut canvas = Canvas::new(720.)
			.gap(7.)
			.push_down(
				&shape::Title,
				shape::Title::from_text(&text::from_data(&data, &data.username, suffix.as_deref())),
			)
			.push_down(
				&shape::Subtitle,
				shape::Subtitle::from_label_str(&LABEL, name),
			)
			.push_down_post_draw(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&sky_block::get_level_format(level),
					&sky_block::get_curr_level_xp(member.leveling.xp),
					&sky_block::get_level_xp(member.leveling.xp),
				),
			)
			.push_right_start(
				&shape::Bubble,
				Body::from_bubble(
					ctx,
					&(networth.value + bank + purse),
					"Networth",
					Paint::Gold,
				),
			)
			.push_down(
				&shape::Bubble,
				Body::from_bubble(ctx, &bank, "Bank", Paint::Gold),
			)
			.push_right_start_post_draw(&status, Body::from_status(ctx, &session));

		let categories = networth
			.categories
			.iter()
			.map(|c| {
				(
					c,
					c.items
						.iter()
						.map(|s| {
							(
								s,
								shape::NetworthSlot(
									if let Some(v) = if s.id.starts_with("ENCHANTED_") {
										MATERIALS.get(&s.id[10..])
									} else if let Some(idx) = s.id.find(':') {
										MATERIALS
											.get(&format!("{}:{}", &s.id[..idx], s.damage))
											.or_else(|| MATERIALS.get(&s.id))
									} else if s.damage != 0 {
										MATERIALS
											.get(&format!("{}:{}", &s.id, s.damage))
											.or_else(|| MATERIALS.get(&s.id))
									} else {
										MATERIALS.get(&s.id)
									} {
										Some(v.image())
									} else {
										tracing::warn!(id = s.id, "unknown item");

										None
									},
									s.count,
								),
							)
						})
						.collect::<Vec<_>>(),
				)
			})
			.collect::<Vec<_>>();

		let default = (&Category::default(), vec![]);

		for i in categories.chunks(2) {
			let (left, left_items) = i.get(0).unwrap();
			let (right, right_items) = i.get(1).unwrap_or(&default);

			let rows = left_items.len().max(right_items.len());

			canvas = canvas
				.push_down_start(&shape::Title, shape::Title::from_category(ctx, left))
				.push_right(&shape::Title, shape::Title::from_category(ctx, right));

			for i in 0..rows {
				let left = left_items.get(i);
				let right = right_items.get(i);

				if let Some((item, slot)) = left {
					canvas = canvas
						.push_down_start_post_draw(slot, Body::empty())
						.push_right(
							&shape::NetworthName,
							Body::build_slice(
								&minecraft_string(&item.name).collect::<Vec<_>>(),
								17.,
								None,
							),
						);
				} else {
					canvas = canvas
						.push_down_start_post_draw(&shape::EmptyNetworthSlot, Body::empty())
						.push_right(&shape::EmptyNetworthName, Body::empty());
				}

				if let Some((item, slot)) = right {
					canvas = canvas.push_right_post_draw(slot, Body::empty()).push_right(
						&shape::NetworthName,
						Body::build_slice(
							&minecraft_string(&item.name).collect::<Vec<_>>(),
							17.,
							None,
						),
					);
				} else {
					canvas = canvas
						.push_right(&shape::EmptyNetworthSlot, Body::empty())
						.push_right(&shape::EmptyNetworthName, Body::empty());
				}
			}
		}

		let mut surface = canvas.build(None, background).unwrap();

		canvas::to_png(&mut surface).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content("Networth calculation is in beta, and may be inaccurate.")
			.components(vec![SkyBlockMode::as_root(
				ctx,
				player.uuid,
				Some(name.to_string()),
				Some(SkyBlockMode::Networth(None)),
			)])
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

#[allow(clippy::too_many_lines)]
pub async fn pets(
	ctx: &context::Context<'_>,
	username: Option<String>,
	profile: Option<String>,
	uuid: Option<Uuid>,
) -> Result<(), Error> {
	let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
	let (player, data, session, skin, suffix) =
		crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

	player.increase_searches(ctx).await?;

	let Some(profile) = (match profile {
		Some(profile) => data.stats.sky_block.profiles.iter().find(|p| p.name == profile),
		None => data.stats.sky_block.profiles.first(),
	}) else {
		return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
	};

	let name = profile.name.as_str();
	let profile = Player::get_skyblock_profile(profile, &data.username).await?;

	let Some(member) = profile.members.get(&player.uuid) else {
		return Err(Error::MemberPlayerNotFound(data.username.clone()));
	};

	let default = vec![];
	let items = member.pets.as_ref().unwrap_or(&default);

	let png: Cow<_> = {
		let status = shape::Status(&session, skin.image());
		let level = sky_block::get_level(member.leveling.xp);
		let progress = shape::WideBubbleProgress(
			sky_block::get_level_progress(member.leveling.xp),
			sky_block::get_colours(level),
		);

		let mut canvas = Canvas::new(720.)
			.gap(7.)
			.push_down(
				&shape::Title,
				shape::Title::from_text(&text::from_data(&data, &data.username, suffix.as_deref())),
			)
			.push_down(
				&shape::Subtitle,
				shape::Subtitle::from_label_str(&LABEL, name),
			)
			.push_down_post_draw(
				&progress,
				shape::WideBubbleProgress::from_level_progress(
					ctx,
					&sky_block::get_level_format(level),
					&sky_block::get_curr_level_xp(member.leveling.xp),
					&sky_block::get_level_xp(member.leveling.xp),
				),
			)
			.push_right_start(
				&canvas::shape::Sidebar,
				canvas::body::Body::new(17., None)
					.append_item(
						&::translate::tr!(ctx, "coins"),
						&canvas::label::ToFormatted::to_formatted_label(&member.coin_purse, ctx),
						&Paint::Gold,
					)
					.append_item(
						&::translate::tr!(ctx, "fairy-souls"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fairy_souls_collected,
							ctx,
						),
						&Paint::Aqua,
					)
					.append_item(
						&::translate::tr!(ctx, "fairy-exchanges"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fairy_exchanges,
							ctx,
						),
						&Paint::LightPurple,
					)
					.append_item(
						&::translate::tr!(ctx, "fishing-treasure"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.fishing_treasure_caught,
							ctx,
						),
						&Paint::Blue,
					)
					.append_item(
						&::translate::tr!(ctx, "zones-visited"),
						&canvas::label::ToFormatted::to_formatted_label(&member.zones_visited, ctx),
						&Paint::Green,
					)
					.append_item(
						&::translate::tr!(ctx, "generators-crafted"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.generators_crafted,
							ctx,
						),
						&Paint::White,
					)
					.append_item(
						&::translate::tr!(ctx, "highest-crit"),
						&canvas::label::ToFormatted::to_formatted_label(
							&member.stats.highest_critical_damage,
							ctx,
						),
						&Paint::Red,
					)
					.build(),
			)
			.push_right_post_draw(&status, Body::from_status(ctx, &session));

		let slots = items
			.iter()
			.map(|s| {
				shape::Slot(
					if let Some(v) = MATERIALS.get(&s.id) {
						Some(v.image())
					} else {
						tracing::warn!(id = s.id, "unknown item");

						None
					},
					1,
				)
			})
			.collect::<Vec<_>>();

		for slot in &slots {
			canvas = canvas.push_checked_post_draw(slot, Body::empty());
		}

		let mut surface = canvas.build(None, background).unwrap();

		canvas::to_png(&mut surface).into()
	};

	ctx.send(
		poise::CreateReply::new()
			.content(crate::tip::random(ctx))
			.components(vec![SkyBlockMode::as_root(
				ctx,
				player.uuid,
				Some(name.to_string()),
				Some(SkyBlockMode::Pets(None)),
			)])
			.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
	)
	.await?;

	Ok(())
}

macro_rules! inventory_command {
	($fn: ident, $key: ident, $mode: ident) => {
		#[allow(clippy::too_many_lines)]
		pub async fn $fn(
			ctx: &context::Context<'_>,
			username: Option<String>,
			profile: Option<String>,
			uuid: Option<Uuid>,
		) -> Result<(), Error> {
			let (_, background) = crate::util::get_format_colour_from_input(ctx).await;
			let (player, data, session, skin, suffix) =
				crate::commands::get_player_data_session_skin_suffix(ctx, uuid, username).await?;

			player.increase_searches(ctx).await?;

			#[rustfmt::skip]
			let Some(profile) = (match profile {
				Some(profile) => data.stats.sky_block.profiles.iter().find(|p| p.name == profile),
				None => data.stats.sky_block.profiles.first(),
			}) else {
				return Err(Error::SkyBlockProfileNotFound(data.username.clone()));
			};

			let name = profile.name.as_str();
			let profile = Player::get_skyblock_profile(profile, &data.username).await?;

			#[rustfmt::skip]
			let Some(member) = profile.members.get(&player.uuid) else {
				return Err(Error::MemberPlayerNotFound(data.username.clone()));
			};

			#[rustfmt::skip]
			let items = member.$key.as_ref().map_or(EMPTY_ROW, |i| i.items.as_slice());

			let png: Cow<_> = {
				let status = shape::Status(&session, skin.image());
				let level = sky_block::get_level(member.leveling.xp);
				let progress = shape::WideBubbleProgress(
					sky_block::get_level_progress(member.leveling.xp),
					sky_block::get_colours(level),
				);

				let ctx = &ctx;
				let mut canvas = Canvas::new(720.)
					.gap(7.)
					.push_down(
						&shape::Title,
						shape::Title::from_text(&text::from_data(
							&data,
							&data.username,
							suffix.as_deref(),
						)),
					)
					.push_down(
						&shape::Subtitle,
						shape::Subtitle::from_label_str(&LABEL, name),
					)
					.push_down_post_draw(
						&progress,
						shape::WideBubbleProgress::from_level_progress(
							ctx,
							&sky_block::get_level_format(level),
							&sky_block::get_curr_level_xp(member.leveling.xp),
							&sky_block::get_level_xp(member.leveling.xp),
						),
					)
					.push_right_start(
						&canvas::shape::Sidebar,
						canvas::body::Body::new(17., None)
							.append_item(
								&::translate::tr!(ctx, "coins"),
								&canvas::label::ToFormatted::to_formatted_label(
									&member.coin_purse,
									ctx,
								),
								&Paint::Gold,
							)
							.append_item(
								&::translate::tr!(ctx, "fairy-souls"),
								&canvas::label::ToFormatted::to_formatted_label(
									&member.fairy_souls_collected,
									ctx,
								),
								&Paint::Aqua,
							)
							.append_item(
								&::translate::tr!(ctx, "fairy-exchanges"),
								&canvas::label::ToFormatted::to_formatted_label(
									&member.fairy_exchanges,
									ctx,
								),
								&Paint::LightPurple,
							)
							.append_item(
								&::translate::tr!(ctx, "fishing-treasure"),
								&canvas::label::ToFormatted::to_formatted_label(
									&member.fishing_treasure_caught,
									ctx,
								),
								&Paint::Blue,
							)
							.append_item(
								&::translate::tr!(ctx, "zones-visited"),
								&canvas::label::ToFormatted::to_formatted_label(
									&member.zones_visited,
									ctx,
								),
								&Paint::Green,
							)
							.append_item(
								&::translate::tr!(ctx, "generators-crafted"),
								&canvas::label::ToFormatted::to_formatted_label(
									&member.generators_crafted,
									ctx,
								),
								&Paint::White,
							)
							.append_item(
								&::translate::tr!(ctx, "highest-crit"),
								&canvas::label::ToFormatted::to_formatted_label(
									&member.stats.highest_critical_damage,
									ctx,
								),
								&Paint::Red,
							)
							.build(),
					)
					.push_right_post_draw(&status, Body::from_status(ctx, &session));

				let slots = items
					.iter()
					.map(|s| {
						s.as_ref().map_or(shape::Slot(None, 0), |s| {
							shape::Slot(
								if let Some(v) = if s.id.starts_with("ENCHANTED_") {
									MATERIALS.get(&s.id[10..])
								} else if let Some(idx) = s.id.find(':') {
									MATERIALS
										.get(&format!("{}:{}", &s.id[..idx], s.damage))
										.or_else(|| MATERIALS.get(&s.id))
								} else if s.damage != 0 {
									MATERIALS
										.get(&format!("{}:{}", &s.id, s.damage))
										.or_else(|| MATERIALS.get(&s.id))
								} else {
									MATERIALS.get(&s.id)
								} {
									Some(v.image())
								} else {
									tracing::warn!(id = s.id, "unknown item");

									None
								},
								s.count,
							)
						})
					})
					.collect::<Vec<_>>();

				for slot in &slots {
					canvas = canvas.push_checked_post_draw(slot, Body::empty());
				}

				let mut surface = canvas.build(None, background).unwrap();

				canvas::to_png(&mut surface).into()
			};

			ctx.send(
				poise::CreateReply::new()
					.content(crate::tip::random(ctx))
					.components(vec![SkyBlockMode::as_root(
						ctx,
						player.uuid,
						Some(name.to_string()),
						Some(SkyBlockMode::$mode(None)),
					)])
					.attachment(CreateAttachment::bytes(png, crate::IMAGE_NAME)),
			)
			.await?;

			Ok(())
		}
	};
}

inventory_command!(inventory, inventory, Inventory);
inventory_command!(enderchest, ender_chest, EnderChest);
inventory_command!(talisman, talisman_bag, Talisman);
inventory_command!(quiver, quiver, Quiver);
inventory_command!(fishing, fishing_bag, Fishing);
inventory_command!(potions, potion_bag, Potions);
inventory_command!(equipment, equipment, Equipment);
inventory_command!(wardrobe, wardrobe, Wardrobe);
inventory_command!(candy, candy, Candy);
inventory_command!(vault, vault, Vault);