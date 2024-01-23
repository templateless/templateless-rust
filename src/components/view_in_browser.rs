use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
pub struct ViewInBrowser {
	id: ComponentId,
	text: String,
}

impl ViewInBrowser {
	pub fn new(text: &str) -> Self {
		Self { id: ComponentId::ViewInBrowser, text: text.to_string() }
	}
}

impl Component for ViewInBrowser {}
