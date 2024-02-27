use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
pub struct ViewInBrowser {
	id: ComponentId,
	text: String,
}

impl ViewInBrowser {
	pub fn new(text: Option<String>) -> Self {
		Self { id: ComponentId::ViewInBrowser, text: text.unwrap_or_default() }
	}
}

impl Component for ViewInBrowser {}
