use serde::Serialize;
use std::collections::HashSet;

use crate::{Content, EmailAddress, Result};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	delete_after: Option<u64>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
	to: HashSet<EmailAddress>,
	subject: String,
	content: Content,
	options: EmailOptions,
}

impl Email {
	pub fn builder() -> Self {
		Self {
			to: HashSet::new(),
			subject: "".to_string(),
			content: Content::builder(),
			options: EmailOptions { delete_after: None },
		}
	}

	pub fn to(mut self, email_address: EmailAddress) -> Self {
		self.to.insert(email_address);
		self
	}

	pub fn to_many(mut self, email_addresses: Vec<EmailAddress>) -> Self {
		for email_address in email_addresses.into_iter() {
			self.to.insert(email_address);
		}

		self
	}

	pub fn subject(mut self, subject: &str) -> Self {
		self.subject = subject.to_string();
		self
	}

	pub fn content(mut self, content: Content) -> Self {
		self.content = content;
		self
	}

	pub fn delete_after(mut self, seconds: u64) -> Self {
		self.options.delete_after = Some(seconds);
		self
	}

	pub fn build(&self) -> Result<Self> {
		Ok(self.clone())
	}
}
