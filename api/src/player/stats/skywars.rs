use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct Stats {
	#[serde(deserialize_with = "super::from_trunc_f32")]
	pub coins: u32,
	pub opals: u16,
	pub heads: u32,
	pub souls: u32,
	#[serde(rename = "cosmetic_tokens")]
	pub tokens: u32,
	#[serde(rename = "skywars_chests", deserialize_with = "super::from_trunc_f32")]
	pub loot_chests: u32,
	pub arrows_shot: u32,
	pub arrows_hit: u32,
	#[serde(rename = "levelFormatted")]
	pub level_fmt: Option<String>,
	#[serde(
		rename = "skywars_experience",
		deserialize_with = "super::from_trunc_f32"
	)]
	pub xp: u32,

	#[serde(flatten)]
	pub overall: OverallStats,
	#[serde(flatten)]
	pub solo_normal: SoloNormalStats,
	#[serde(flatten)]
	pub solo_insane: SoloInsaneStats,
	#[serde(flatten)]
	pub team_normal: TeamNormalStats,
	#[serde(flatten)]
	pub team_insane: TeamInsaneStats,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct OverallStats {
	pub losses: u32,
	pub wins: u32,
	pub kills: u32,
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SoloNormalStats {
	#[serde(rename = "losses_solo_normal")]
	pub losses: u32,
	#[serde(rename = "wins_solo_normal")]
	pub wins: u32,
	#[serde(rename = "kills_solo_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_normal")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct SoloInsaneStats {
	#[serde(rename = "losses_solo_insane")]
	pub losses: u32,
	#[serde(rename = "wins_solo_insane")]
	pub wins: u32,
	#[serde(rename = "kills_solo_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_solo_insane")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct TeamNormalStats {
	#[serde(rename = "losses_team_normal")]
	pub losses: u32,
	#[serde(rename = "wins_team_normal")]
	pub wins: u32,
	#[serde(rename = "kills_team_normal")]
	pub kills: u32,
	#[serde(rename = "deaths_team_normal")]
	pub deaths: u32,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct TeamInsaneStats {
	#[serde(rename = "losses_team_insane")]
	pub losses: u32,
	#[serde(rename = "wins_team_insane")]
	pub wins: u32,
	#[serde(rename = "kills_team_insane")]
	pub kills: u32,
	#[serde(rename = "deaths_team_insane")]
	pub deaths: u32,
}
