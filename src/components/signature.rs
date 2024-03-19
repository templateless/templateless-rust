use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Font {
	#[default]
	ReenieBeanie = 1,
	MeowScript = 2,
	Caveat = 3,
	Zeyada = 4,
	Petemoss = 5,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
	id: ComponentId,
	text: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	font: Option<Font>,
}

impl Signature {
	pub fn new(text: &str, font: Option<Font>) -> Self {
		Self { id: ComponentId::Signature, text: text.to_string(), font }
	}
}

impl Component for Signature {}
