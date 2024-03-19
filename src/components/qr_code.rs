use base64::prelude::*;
use derive_more::Display;
use serde::Serialize;

use crate::components::{Component, ComponentId};

#[derive(Display)]
pub enum Cryptocurrency {
	#[display(fmt = "bitcoin")]
	Bitcoin,
	#[display(fmt = "ethereum")]
	Ethereum,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCode {
	id: ComponentId,
	data: String,
}

impl QrCode {
	pub fn new(data: &[u8]) -> Self {
		Self { id: ComponentId::QrCode, data: BASE64_STANDARD.encode(data) }
	}

	pub fn email(email: &str) -> Self {
		Self::new(format!("mailto:{email}").as_bytes())
	}

	pub fn url(url: &str) -> Self {
		Self::new(url.as_bytes())
	}

	pub fn phone(phone: &str) -> Self {
		Self::new(format!("tel:{phone}").as_bytes())
	}

	pub fn sms(text: &str) -> Self {
		Self::new(format!("smsto:{text}").as_bytes())
	}

	pub fn coordinates(lat: f64, lng: f64) -> Self {
		Self::new(format!("geo:{lat},{lng}").as_bytes())
	}

	pub fn cryptocurrency_address(
		cryptocurrency: Cryptocurrency,
		address: &str,
	) -> Self {
		Self::new(format!("{cryptocurrency}:{address}").as_bytes())
	}
}

impl Component for QrCode {}
