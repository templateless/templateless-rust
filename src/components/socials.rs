use crate::components::Component;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Eq, PartialEq, Hash)]
pub enum Service {
	#[serde(rename = "website")]
	Website,
	#[serde(rename = "email")]
	Email,
	#[serde(rename = "facebook")]
	Facebook,
	#[serde(rename = "youtube")]
	YouTube,
	#[serde(rename = "twitter")]
	Twitter,
	#[serde(rename = "x")]
	X,
	#[serde(rename = "github")]
	Github,
	#[serde(rename = "instagram")]
	Instagram,
	#[serde(rename = "linkedin")]
	LinkedIn,
	#[serde(rename = "slack")]
	Slack,
	#[serde(rename = "discord")]
	Discord,
	#[serde(rename = "tiktok")]
	TikTok,
	#[serde(rename = "snapchat")]
	Snapchat,
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
	id: String,
	data: Vec<Item>,
}

impl Socials {
	pub fn new(data: Vec<Item>) -> Self {
		Self { id: "socials".to_string(), data }
	}
}

impl Component for Socials {}
