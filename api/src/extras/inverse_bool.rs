use serde::{Deserialize, Deserializer};
use translate::Context;

use crate::canvas::label::ToFormatted;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct InverseBool(bool);

impl<'de> Deserialize<'de> for InverseBool {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: bool = Deserialize::deserialize(deserializer)?;

		Ok(InverseBool(!s))
	}
}

impl serde::Serialize for InverseBool {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_bool(!self.0)
	}
}

impl ToFormatted for InverseBool {
	fn to_formatted_label(&self, ctx: Context<'_>, percent: bool) -> String {
		(!self.0).to_formatted_label(ctx, percent)
	}
}