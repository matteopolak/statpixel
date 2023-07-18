pub use super::{convert, get_colours, get_level_format, get_total_xp, Level};

const TOTAL_XP: [u32; 47] = [
	0,
	50 * 2,
	60 * 2,
	70 * 2,
	80 * 2,
	90 * 2,
	100 * 2,
	130 * 2,
	160 * 2,
	190 * 2,
	220 * 2,
	250 * 2,
	300 * 2,
	350 * 2,
	400 * 2,
	450 * 2,
	500 * 2,
	600 * 2,
	700 * 2,
	800 * 2,
	900 * 2,
	1_000 * 2,
	1_200 * 2,
	1_400 * 2,
	1_600 * 2,
	1_800 * 2,
	2_000 * 2,
	2_600 * 2,
	3_200 * 2,
	3_800 * 2,
	4_400 * 2,
	5_000 * 2,
	6_000 * 2,
	7_000 * 2,
	8_000 * 2,
	9_000 * 2,
	10_000 * 2,
	13_000 * 2,
	16_000 * 2,
	19_000 * 2,
	22_000 * 2,
	25_000 * 2,
	30_000 * 2,
	35_000 * 2,
	40_000 * 2,
	45_000 * 2,
	50_000 * 2,
];
const XP_PER_LEVEL: u32 = 20_000;

#[must_use]
pub fn get_level(xp: u32) -> Level {
	Level(match xp {
		0..100 => 0,
		100..120 => 1,
		120..140 => 2,
		140..160 => 3,
		160..180 => 4,
		180..200 => 5,
		200..260 => 6,
		260..320 => 7,
		320..380 => 8,
		380..440 => 9,
		440..500 => 10,
		500..600 => 11,
		600..700 => 12,
		700..800 => 13,
		800..900 => 14,
		900..1_000 => 15,
		1_000..1_200 => 16,
		1_200..1_400 => 17,
		1_400..1_600 => 18,
		1_600..1_800 => 19,
		1_800..2_000 => 20,
		2_000..2_400 => 21,
		2_400..2_800 => 22,
		2_800..3_200 => 23,
		3_200..3_600 => 24,
		3_600..4_000 => 25,
		4_000..5_200 => 26,
		5_200..6_400 => 27,
		6_400..7_600 => 28,
		7_600..8_800 => 29,
		8_800..10_000 => 30,
		10_000..12_000 => 31,
		12_000..14_000 => 32,
		14_000..16_000 => 33,
		16_000..18_000 => 34,
		18_000..20_000 => 35,
		20_000..26_000 => 36,
		26_000..32_000 => 37,
		32_000..38_000 => 38,
		38_000..44_000 => 39,
		44_000..50_000 => 40,
		50_000..60_000 => 41,
		60_000..70_000 => 42,
		70_000..80_000 => 43,
		80_000..90_000 => 44,
		90_000..100_000 => 45,
		_ => 46 + (xp - 100_000) / XP_PER_LEVEL,
	})
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn get_xp(level: Level) -> u32 {
	if (level.0 as usize) < TOTAL_XP.len() {
		TOTAL_XP[level.0 as usize]
	} else {
		TOTAL_XP[TOTAL_XP.len() - 1] + (level.0 - TOTAL_XP.len() as u32) * XP_PER_LEVEL
	}
}

#[must_use]
pub fn get_level_xp(xp: u32) -> u32 {
	let level = get_level(xp);

	get_xp(Level(level.0 + 1)) - get_xp(level)
}

#[must_use]
pub fn get_curr_level_xp(xp: u32) -> u32 {
	xp - get_xp(get_level(xp))
}

#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn get_level_progress(xp: u32) -> f32 {
	let level = get_level(xp);
	let base = get_xp(level);
	let next = get_xp(Level(level.0 + 1));

	(xp - base) as f32 / (next - base) as f32
}
