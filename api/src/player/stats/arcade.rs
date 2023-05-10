use macros::Game;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone, Game)]
#[game(
	path = "arcade",
	pretty = "§b§lArcade",
	field(ident = "wins", colour = "green"),
	field(ident = "games", colour = "red"),
	field(tr = "wr", ident = "wins", div = "games", colour = "gold", percent),
	field(ident = "kills", colour = "green"),
	field(ident = "deaths", colour = "red"),
	field(tr = "kdr", ident = "kills", div = "deaths", colour = "gold")
)]
#[serde(default)]
pub struct Arcade {
	#[serde(deserialize_with = "super::from_trunc_f32_to_u32")]
	#[game(label(colour = "gold"))]
	pub coins: u32,
	#[serde(rename = "mystery_gifts_obtained")]
	#[game(label(colour = "yellow"))]
	pub mystery_gifts: u32,

	#[serde(flatten)]
	#[game(mode())]
	pub party: Party,
	#[serde(flatten)]
	#[game(mode())]
	pub santa_says: SantaSays,
	#[serde(flatten)]
	#[game(mode())]
	pub simon_says: SimonSays,
	#[serde(flatten)]
	#[game(mode())]
	pub mini_walls: MiniWalls,
	#[serde(flatten)]
	#[game(mode())]
	pub soccer: Soccer,
	#[serde(flatten)]
	#[game(mode())]
	pub one_in_the_quiver: OneInTheQuiver,
	#[serde(flatten)]
	#[game(mode())]
	pub ender_spleef: EnderSpleef,
	#[serde(flatten)]
	#[game(mode())]
	pub farm_hunt: FarmHunt,
	#[serde(flatten)]
	#[game(mode())]
	pub dragon_wars: DragonWars,
	#[serde(flatten)]
	#[game(mode())]
	pub blocking_dead: BlockingDead,
	#[serde(flatten)]
	#[game(mode())]
	pub zombies: Zombies,
	#[serde(flatten)]
	#[game(mode())]
	pub zombies_bad_blood: ZombiesBadBlood,
	#[serde(flatten)]
	#[game(mode())]
	pub zombies_dead_end: ZombiesDeadEnd,
	#[serde(flatten)]
	#[game(mode())]
	pub pixel_painters: PixelPainters,
	#[serde(flatten)]
	#[game(mode())]
	pub hole_in_the_wall: HoleInTheWall,
	#[serde(flatten)]
	#[game(mode())]
	pub throw_out: ThrowOut,
	#[serde(flatten)]
	#[game(mode())]
	pub easter_simulator: EasterSimulator,
	#[serde(flatten)]
	#[game(mode())]
	pub scuba_simulator: ScubaSimulator,
	#[serde(flatten)]
	#[game(mode())]
	pub halloween_simulator: HalloweenSimulator,
	#[serde(flatten)]
	#[game(mode())]
	pub grinch_simulator: GrinchSimulator,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Party {
	#[serde(rename = "rounds_party")]
	pub games: u32,
	#[serde(rename = "wins_party")]
	pub wins: u32,
	#[serde(rename = "kills_party")]
	pub kills: u32,
	#[serde(rename = "deaths_party")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SantaSays {
	#[serde(rename = "rounds_santa_says")]
	pub games: u32,
	#[serde(rename = "wins_santa_says")]
	pub wins: u32,
	#[serde(rename = "kills_santa_says")]
	pub kills: u32,
	#[serde(rename = "deaths_santa_says")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SimonSays {
	#[serde(rename = "rounds_simon_says")]
	pub games: u32,
	#[serde(rename = "wins_simon_says")]
	pub wins: u32,
	#[serde(rename = "kills_simon_says")]
	pub kills: u32,
	#[serde(rename = "deaths_simon_says")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct MiniWalls {
	#[serde(rename = "rounds_mini_walls")]
	pub games: u32,
	#[serde(rename = "wins_mini_walls")]
	pub wins: u32,
	#[serde(rename = "kills_mini_walls")]
	pub kills: u32,
	#[serde(rename = "deaths_mini_walls")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Soccer {
	#[serde(rename = "rounds_soccer")]
	pub games: u32,
	#[serde(rename = "wins_soccer")]
	pub wins: u32,
	#[serde(rename = "kills_soccer")]
	pub kills: u32,
	#[serde(rename = "deaths_soccer")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct OneInTheQuiver {
	#[serde(rename = "rounds_oneinthequiver")]
	pub games: u32,
	#[serde(rename = "wins_oneinthequiver")]
	pub wins: u32,
	#[serde(rename = "kills_oneinthequiver")]
	pub kills: u32,
	#[serde(rename = "deaths_oneinthequiver")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct EnderSpleef {
	#[serde(rename = "rounds_ender")]
	pub games: u32,
	#[serde(rename = "wins_ender")]
	pub wins: u32,
	#[serde(rename = "kills_ender")]
	pub kills: u32,
	#[serde(rename = "deaths_ender")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct FarmHunt {
	#[serde(rename = "rounds_farm_hunt")]
	pub games: u32,
	#[serde(rename = "wins_farm_hunt")]
	pub wins: u32,
	#[serde(rename = "kills_farm_hunt")]
	pub kills: u32,
	#[serde(rename = "deaths_farm_hunt")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct DragonWars {
	#[serde(rename = "rounds_dragonwars2")]
	pub games: u32,
	#[serde(rename = "wins_dragonwars2")]
	pub wins: u32,
	#[serde(rename = "kills_dragonwars2")]
	pub kills: u32,
	#[serde(rename = "deaths_dragonwars2")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct BlockingDead {
	#[serde(rename = "rounds_dayone")]
	pub games: u32,
	#[serde(rename = "wins_dayone")]
	pub wins: u32,
	#[serde(rename = "kills_dayone")]
	pub kills: u32,
	#[serde(rename = "deaths_dayone")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Zombies {
	#[serde(rename = "rounds_zombies")]
	pub games: u32,
	#[serde(rename = "wins_zombies")]
	pub wins: u32,
	#[serde(rename = "kills_zombies")]
	pub kills: u32,
	#[serde(rename = "deaths_zombies")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct ZombiesBadBlood {
	#[serde(rename = "rounds_zombies_badblood")]
	pub games: u32,
	#[serde(rename = "wins_zombies_badblood")]
	pub wins: u32,
	#[serde(rename = "kills_zombies_badblood")]
	pub kills: u32,
	#[serde(rename = "deaths_zombies_badblood")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct ZombiesDeadEnd {
	#[serde(rename = "rounds_zombies_deadend")]
	pub games: u32,
	#[serde(rename = "wins_zombies_deadend")]
	pub wins: u32,
	#[serde(rename = "kills_zombies_deadend")]
	pub kills: u32,
	#[serde(rename = "deaths_zombies_deadend")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct PixelPainters {
	#[serde(rename = "rounds_draw_their_thing")]
	pub games: u32,
	#[serde(rename = "wins_draw_their_thing")]
	pub wins: u32,
	#[serde(rename = "kills_draw_their_thing")]
	pub kills: u32,
	#[serde(rename = "deaths_draw_their_thing")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Paintball {
	#[serde(rename = "rounds_paintball")]
	pub games: u32,
	#[serde(rename = "wins_paintball")]
	pub wins: u32,
	#[serde(rename = "kills_paintball")]
	pub kills: u32,
	#[serde(rename = "deaths_paintball")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct HoleInTheWall {
	#[serde(rename = "rounds_hole_in_the_wall")]
	pub games: u32,
	#[serde(rename = "wins_hole_in_the_wall")]
	pub wins: u32,
	#[serde(rename = "kills_hole_in_the_wall")]
	pub kills: u32,
	#[serde(rename = "deaths_hole_in_the_wall")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct ThrowOut {
	#[serde(rename = "rounds_throw_out")]
	pub games: u32,
	#[serde(rename = "wins_throw_out")]
	pub wins: u32,
	#[serde(rename = "kills_throw_out")]
	pub kills: u32,
	#[serde(rename = "deaths_throw_out")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct EasterSimulator {
	#[serde(rename = "rounds_easter_simulator")]
	pub games: u32,
	#[serde(rename = "wins_easter_simulator")]
	pub wins: u32,
	#[serde(rename = "kills_easter_simulator")]
	pub kills: u32,
	#[serde(rename = "deaths_easter_simulator")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct ScubaSimulator {
	#[serde(rename = "rounds_scuba_simulator")]
	pub games: u32,
	#[serde(rename = "wins_scuba_simulator")]
	pub wins: u32,
	#[serde(rename = "kills_scuba_simulator")]
	pub kills: u32,
	#[serde(rename = "deaths_scuba_simulator")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct HalloweenSimulator {
	#[serde(rename = "rounds_halloween_simulator")]
	pub games: u32,
	#[serde(rename = "wins_halloween_simulator")]
	pub wins: u32,
	#[serde(rename = "kills_halloween_simulator")]
	pub kills: u32,
	#[serde(rename = "deaths_halloween_simulator")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct GrinchSimulator {
	#[serde(rename = "rounds_grinch_simulator_v2")]
	pub games: u32,
	#[serde(rename = "wins_grinch_simulator_v2")]
	pub wins: u32,
	#[serde(rename = "kills_grinch_simulator_v2")]
	pub kills: u32,
	#[serde(rename = "deaths_grinch_simulator_v2")]
	pub deaths: u32,
}
