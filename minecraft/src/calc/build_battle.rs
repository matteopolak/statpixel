use skia_safe::Color;

use crate::{colour::Colour, text::parse::ESCAPE};

pub fn get_level_format(level: u32) -> String {
	match level {
		0..=1 => format!("{ESCAPE}fRookie"),
		2 => format!("{ESCAPE}8Untrained"),
		3 => format!("{ESCAPE}eAmateur"),
		4 => format!("{ESCAPE}aApprentice"),
		5 => format!("{ESCAPE}dExperienced"),
		6 => format!("{ESCAPE}9Seasoned"),
		7 => format!("{ESCAPE}2Trained"),
		8 => format!("{ESCAPE}3Skilled"),
		9 => format!("{ESCAPE}cTalented"),
		10 => format!("{ESCAPE}5Professional"),
		11 => format!("{ESCAPE}1Expert"),
		12 => format!("{ESCAPE}4Master"),
		_ => format!("{ESCAPE}6#1 Builder"),
	}
}

pub fn get_colours(level: u32) -> [Color; 2] {
	let colour = match level {
		0..=1 => Colour::White,
		2 => Colour::DarkGray,
		3 => Colour::Yellow,
		4 => Colour::Green,
		5 => Colour::LightPurple,
		6 => Colour::Blue,
		7 => Colour::DarkGreen,
		8 => Colour::DarkAqua,
		9 => Colour::Red,
		10 => Colour::DarkPurple,
		11 => Colour::DarkBlue,
		12 => Colour::DarkRed,
		_ => Colour::Gold,
	};

	[colour.into(), colour.into()]
}

pub fn get_level(xp: u32) -> u32 {
	match xp {
		0..100 => 1,
		100..250 => 2,
		250..500 => 3,
		500..1_000 => 4,
		1_000..2_000 => 5,
		2_000..3_500 => 6,
		3_500..5_000 => 7,
		5_000..7_500 => 8,
		7_500..10_000 => 9,
		10_000..15_000 => 10,
		15_000..20_000 => 11,
		// TODO: add "#1 Builder" level
		_ => 12,
	}
}

pub fn get_xp(level: u32) -> u32 {
	match level {
		..=1 => 0,
		2 => 100,
		3 => 250,
		4 => 500,
		5 => 1_000,
		6 => 2_000,
		7 => 3_500,
		8 => 5_000,
		9 => 7_500,
		10 => 10_000,
		11 => 15_000,
		12.. => 20_000,
	}
}

pub fn get_level_xp(xp: u32) -> u32 {
	let level = get_level(xp);

	get_xp(level + 1) - get_xp(level)
}

pub fn get_curr_level_xp(xp: u32) -> u32 {
	xp - get_xp(get_level(xp))
}

pub fn get_level_progress(xp: u32) -> f32 {
	let level = get_level(xp);
	let base = get_xp(level);
	let next = get_xp(level + 1);

	(xp - base) as f32 / (next - base) as f32
}
