use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Eq, PartialEq, Hash)]
pub enum Style {
	#[serde(rename = "none")]
	None,
}

#[derive(Clone, Serialize)]
pub struct List {
	id: String,
	style: Style,
	items: Vec<Box<dyn Component>>,
}

impl List {
	pub fn new(style: Style, items: Vec<Box<dyn Component>>) -> Self {
		Self { id: "list".to_string(), style, items }
	}
}

impl Component for List {}
