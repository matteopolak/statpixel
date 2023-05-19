use std::borrow::Cow;

use super::{label::ToFormatted, GAP, HEADER_LEFT_END_X, ITEM_WIDTH, PADDING};

use minecraft::{
	paint::{self, Paint},
	text::{draw, Text},
};
use skia_safe::{textlayout::TextAlign, Rect, Surface};
use translate::Context;

pub type Line<'c> = (Cow<'c, str>, Box<dyn ToFormatted>, Paint, bool);

/// (name, value, colour, percent)
pub fn item(ctx: Context<'_>, surface: &mut Surface, line: &Line<'_>, idx: u8) {
	let y = PADDING + f32::from(idx) * 21.2;
	let x = HEADER_LEFT_END_X + GAP;

	let rect = Rect::from_xywh(x, y, ITEM_WIDTH, 21.2).with_offset((17., 13.));
	let text = line.1.to_formatted_label(ctx, line.3);

	draw(
		surface,
		&[
			Text {
				text: "• ",
				paint: line.2,
				..Default::default()
			},
			Text {
				text: &format!("{}: ", line.0),
				paint: paint::Paint::White,
				..Default::default()
			},
			Text {
				text: &text,
				paint: line.2,
				..Default::default()
			},
		],
		17.,
		rect,
		TextAlign::Left,
		true,
	);
}
