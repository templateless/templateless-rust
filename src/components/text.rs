use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
	id: ComponentId,
	text: String,
}

impl Text {
	pub fn new(text: &str) -> Self {
		Self { id: ComponentId::Text, text: text.to_string() }
	}
}

impl Component for Text {}
