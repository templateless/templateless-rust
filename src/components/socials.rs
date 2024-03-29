use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Clone, Debug, Serialize, Eq, PartialEq, Hash)]
pub enum Service {
	#[serde(rename = "WEBSITE")]
	Website,
	#[serde(rename = "EMAIL")]
	Email,
	#[serde(rename = "PHONE")]
	Phone,
	#[serde(rename = "FACEBOOK")]
	Facebook,
	#[serde(rename = "YOUTUBE")]
	YouTube,
	#[serde(rename = "TWITTER")]
	Twitter,
	#[serde(rename = "X")]
	X,
	#[serde(rename = "GITHUB")]
	GitHub,
	#[serde(rename = "INSTAGRAM")]
	Instagram,
	#[serde(rename = "LINKEDIN")]
	LinkedIn,
	#[serde(rename = "SLACK")]
	Slack,
	#[serde(rename = "DISCORD")]
	Discord,
	#[serde(rename = "TIKTOK")]
	TikTok,
	#[serde(rename = "SNAPCHAT")]
	Snapchat,
	#[serde(rename = "THREADS")]
	Threads,
	#[serde(rename = "TELEGRAM")]
	Telegram,
	#[serde(rename = "MASTODON")]
	Mastodon,
	#[serde(rename = "RSS")]
	Rss,
}

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
	key: Service,
	value: String,
}

impl Item {
	pub fn new(key: Service, value: &str) -> Self {
		Self { key, value: value.to_string() }
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
