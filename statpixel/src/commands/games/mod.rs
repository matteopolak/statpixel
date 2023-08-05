pub mod image;
pub mod run;

#[allow(clippy::wildcard_imports)]
use api::player::stats;

use crate::util;
use translate::{context, Context, Error};

#[rustfmt::skip]
macro_rules! large_command {
	($game: ty, $fn: ident, $name: literal) => {
		pub mod $fn {
			use super::*;
			use crate::commands::from::$fn::command as from;
			use crate::commands::history::$fn as history;
			use crate::commands::project::$fn::command as project;
			use crate::commands::snapshot::daily::$fn as daily;
			use crate::commands::snapshot::monthly::$fn as monthly;
			use crate::commands::snapshot::weekly::$fn as weekly;

			async fn autocomplete_mode<'a>(
				ctx: $crate::Context<'a>,
				partial: &'a str,
			) -> impl ::futures::Stream<Item = ::poise::AutocompleteChoice<u32>> + 'a {
				let partial = partial.to_ascii_lowercase();

				<$game>::autocomplete(ctx, partial).await
			}

			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES"
			)]
			async fn general(
				ctx: $crate::Context<'_>,
				#[max_length = 16]
				#[autocomplete = "crate::commands::autocomplete_username"]
				username: Option<::std::string::String>,
				#[min_length = 32]
				#[max_length = 36]
				uuid: Option<::std::string::String>,
				#[autocomplete = "autocomplete_mode"] mode: Option<u32>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let mode: ::std::option::Option<<$game as api::prelude::Game>::Mode> =
					mode.map(|m| m.into());
				let uuid = util::parse_uuid(uuid.as_deref())?;
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, username, uuid, mode).await
			}

			#[allow(clippy::unused_async)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				subcommands("general", "from", "daily", "weekly", "monthly", "history", "project"),
				rename = $name
			)]
			pub async fn parent(_ctx: Context<'_>) -> Result<(), Error> {
				Ok(())
			}
		}
	};
}

#[rustfmt::skip]
macro_rules! command {
	($game: ty, $fn: ident, $name: literal) => {
		pub mod $fn {
			use super::*;
			use crate::commands::from::$fn as from;
			use crate::commands::history::$fn as history;
			use crate::commands::project::$fn::command as project;
			use crate::commands::snapshot::daily::$fn as daily;
			use crate::commands::snapshot::monthly::$fn as monthly;
			use crate::commands::snapshot::weekly::$fn as weekly;

			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES"
			)]
			async fn general(
				ctx: $crate::Context<'_>,
				#[max_length = 16]
				#[autocomplete = "crate::commands::autocomplete_username"]
				username: Option<::std::string::String>,
				#[min_length = 32]
				#[max_length = 36]
				uuid: Option<::std::string::String>,
				mode: Option<<$game as api::prelude::Game>::Mode>,
			) -> ::std::result::Result<(), ::translate::Error> {
				let uuid = util::parse_uuid(uuid.as_deref())?;
				let ctx = &context::Context::from_poise(&ctx);

				run::command::<$game>(ctx, username, uuid, mode).await
			}

			#[allow(clippy::unused_async)]
			#[poise::command(
				on_error = "crate::util::error_handler",
				slash_command,
				required_bot_permissions = "ATTACH_FILES",
				subcommands("general", "from", "daily", "weekly", "monthly", "history", "project"),
				rename = $name
			)]
			pub async fn parent(_ctx: Context<'_>) -> Result<(), Error> {
				Ok(())
			}
		}
	};
}

large_command!(stats::arcade::Arcade, arcade, "arcade");
command!(stats::arena::Arena, arena, "arena");
large_command!(stats::bed_wars::BedWars, bedwars, "bedwars");
large_command!(stats::blitz_sg::BlitzSg, blitz, "blitz");
command!(stats::build_battle::BuildBattle, buildbattle, "buildbattle");
command!(
	stats::cops_and_crims::CopsAndCrims,
	copsandcrims,
	"copsandcrims"
);
large_command!(stats::duels::Duels, duels, "duels");
command!(stats::mega_walls::MegaWalls, megawalls, "megawalls");
command!(
	stats::murder_mystery::MurderMystery,
	murdermystery,
	"murdermystery"
);
command!(stats::paintball::Paintball, paintball, "paintball");
command!(stats::pit::Pit, pit, "pit");
command!(stats::quake::Quake, quake, "quake");
command!(stats::sky_wars::SkyWars, skywars, "skywars");
command!(stats::smash_heroes::SmashHeroes, smash, "smash");
command!(stats::speed_uhc::SpeedUhc, speeduhc, "speeduhc");
command!(stats::tnt_games::TntGames, tntgames, "tntgames");
command!(
	stats::turbo_kart_racers::TurboKartRacers,
	turbokartracers,
	"turbokartracers"
);
command!(stats::uhc::Uhc, uhc, "uhc");
command!(stats::vampire_z::VampireZ, vampirez, "vampirez");
command!(stats::walls::Walls, walls, "walls");
command!(stats::warlords::Warlords, warlords, "warlords");
command!(stats::wool_wars::WoolWars, woolwars, "woolwars");
