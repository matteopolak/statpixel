use std::ops::Add;

use serde::{Deserialize, Deserializer};
use translate::Context;

use crate::canvas::label::ToFormatted;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Meters(u64);

impl<'de> Deserialize<'de> for Meters {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: u64 = Deserialize::deserialize(deserializer)?;

		Ok(Meters(s))
	}
}

impl serde::Serialize for Meters {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_u64(self.0)
	}
}

impl Add for Meters {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Meters(self.0 + rhs.0)
	}
}

impl From<Meters> for u64 {
	fn from(s: Meters) -> Self {
		s.0
	}
}

impl ToFormatted for Meters {
	#[allow(clippy::cast_precision_loss)]
	fn to_formatted_label(&self, ctx: Context<'_>, _percent: bool) -> String {
		let m = self.0;

		if let 0..1_000 = m {
			return format!("{}m", m.to_formatted_label(ctx, false));
		}

		format!("{}km", (m as f64 / 1_000.).to_formatted_label(ctx, false))
	}
}