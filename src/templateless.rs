use console::style;
use dotenvy::dotenv;
use reqwest::{
	header::{AUTHORIZATION, REFERER},
	Client, StatusCode,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
	errors::BadRequestCode, Email, Error, ObjectId, Result, TEMPLATELESS_DOMAIN,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailResponsePreview {
	pub preview: String,
	pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailResponse {
	pub emails: Vec<ObjectId>,
	pub previews: Option<Vec<EmailResponsePreview>>,
}

pub struct Templateless {
	api_key: String,
	domain: String,
}

impl Templateless {
	pub fn new(api_key: &str) -> Self {
		dotenv().ok();

		let domain = match dotenvy::var("TEMPLATELESS_DOMAIN") {
			Ok(domain) if !domain.is_empty() => domain,
			_ => TEMPLATELESS_DOMAIN.to_string(),
		};

		Self { api_key: api_key.to_string(), domain }
	}

	pub fn domain(mut self, domain: &str) -> Self {
		self.domain = domain.to_string();
		self
	}

	pub async fn send(&self, email: Email) -> Result<Vec<ObjectId>> {
		self.send_many(vec![email]).await
	}

	pub async fn send_many(&self, emails: Vec<Email>) -> Result<Vec<ObjectId>> {
		let response = Client::new()
			.post(format!("{}/v1/emails", self.domain))
			.header(AUTHORIZATION, format!("Bearer {}", self.api_key))
			.header(
				REFERER,
				format!(
					"{}-rust v{}",
					env!("CARGO_PKG_NAME"),
					env!("CARGO_PKG_VERSION")
				),
			)
			.json(&HashMap::from([("payload", emails)]))
			.send()
			.await
			.map_err(|_| Error::Unavailable)?;

		match response.status() {
			StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
			StatusCode::FORBIDDEN => Err(Error::Forbidden),
			StatusCode::UNPROCESSABLE_ENTITY => Err({
				let response_text =
					response.text().await.map_err(|_| Error::Unknown)?;

				#[derive(Deserialize)]
				struct InvalidParameter {
					field: String,
				}

				let ret: InvalidParameter =
					serde_json::from_str(&response_text)
						.map_err(|_| Error::Unknown)?;

				Error::InvalidParameter { field: ret.field }
			}),
			StatusCode::BAD_REQUEST => Err({
				let response_text =
					response.text().await.map_err(|_| Error::Unknown)?;

				#[derive(Deserialize)]
				struct BadRequest {
					code: BadRequestCode,
					error: String,
				}

				let ret: BadRequest = serde_json::from_str(&response_text)
					.map_err(|_| Error::Unknown)?;

				Error::BadRequest { code: ret.code, error: ret.error }
			}),
			StatusCode::INTERNAL_SERVER_ERROR => Err(Error::Unavailable),
			StatusCode::OK => {
				let response_text =
					response.text().await.map_err(|_| Error::Unknown)?;

				let ret: EmailResponse = serde_json::from_str(&response_text)
					.map_err(|_| Error::Unknown)?;

				if let Some(previews) = &ret.previews {
					for preview in previews.iter() {
						println!(
							"{} {}: Emailed {}, preview: {}",
							style("Templateless").bold(),
							style("[TEST MODE]").bold().yellow(),
							style(&preview.email).italic(),
							style(format!(
								"https://tmpl.sh/{}",
								preview.preview
							))
							.bold()
							.underlined(),
						);
					}
				}

				Ok(ret.emails)
			}
			_ => Err(Error::Unknown),
		}
	}
}
