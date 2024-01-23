use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Debug, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Service {
	Website,
	Email,
	Twitter,
	X,
	Github,
}

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq)]
pub struct Item {
	service: Service,
	value: String,
}

impl Item {
	pub fn new(service: Service, value: &str) -> Self {
		Self { service, value: value.to_string() }
	}
}

#[derive(Clone, Serialize)]
pub struct Socials {
	id: ComponentId,
	data: Vec<Item>,
}

impl Socials {
	pub fn new(data: Vec<Item>) -> Self {
		Self { id: ComponentId::Socials, data }
	}
}

impl Component for Socials {}
