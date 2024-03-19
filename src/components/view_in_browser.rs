use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewInBrowser {
	id: ComponentId,
	text: Option<String>,
}

impl ViewInBrowser {
	pub fn new(text: Option<String>) -> Self {
		Self { id: ComponentId::ViewInBrowser, text }
	}
}

impl Component for ViewInBrowser {}
