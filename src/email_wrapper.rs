use dotenvy::dotenv;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{Email, EmailWrapperResult, Error, ObjectId, EMAILWRAPPER_DOMAIN};

#[derive(Debug, Deserialize)]
pub struct EmailResponse {
	pub emails: Vec<ObjectId>,
}

pub struct EmailWrapper {
	api_key: String,
	domain: String,
}

impl EmailWrapper {
	pub fn new(api_key: &str) -> Self {
		dotenv().ok();

		let domain = match dotenvy::var("EMAILWRAPPER_DOMAIN") {
			Ok(domain) if !domain.is_empty() => domain,
			_ => EMAILWRAPPER_DOMAIN.to_string(),
		};

		Self { api_key: api_key.to_string(), domain }
	}

	pub fn domain(&mut self, domain: &str) -> &mut Self {
		self.domain = domain.to_string();
		self
	}

	pub async fn send(
		&self,
		email: Email,
	) -> EmailWrapperResult<Vec<ObjectId>> {
		self.send_many(vec![email]).await
	}

	pub async fn send_many(
		&self,
		emails: Vec<Email>,
	) -> EmailWrapperResult<Vec<ObjectId>> {
		let response = Client::new()
			.post(format!("{}/v1/email", self.domain))
			.header("Authorization", format!("Bearer {}", self.api_key))
			.json(&HashMap::from([("payload", emails)]))
			.send()
			.await
			.map_err(|_| Error::Unavailable)?;

		match response.status() {
			StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
			_ => {
				let response_text =
					response.text().await.map_err(|_| Error::Unknown)?;

				let ret: EmailResponse = serde_json::from_str(&response_text)
					.map_err(|_| Error::Unknown)?;

				Ok(ret.emails)
			}
		}
	}
}
