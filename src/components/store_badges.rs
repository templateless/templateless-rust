use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Serialize)]
pub enum StoreBadge {
	#[serde(rename = "APP_STORE")]
	AppStore,
	#[serde(rename = "GOOGLE_PLAY")]
	GooglePlay,
	#[serde(rename = "MICROSOFT_STORE")]
	MicrosoftStore,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
	key: StoreBadge,
	value: String,
}

impl Item {
	pub fn new(key: StoreBadge, value: &str) -> Self {
		Self { key, value: value.to_string() }
	}
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreBadges {
	id: ComponentId,
	data: Vec<Item>,
}

impl StoreBadges {
	pub fn new(data: Vec<Item>) -> Self {
		Self { id: ComponentId::StoreBadges, data }
	}
}

impl Component for StoreBadges {}
