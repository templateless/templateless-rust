use serde::Serialize;
use std::collections::HashSet;

use crate::{Content, EmailAddress};

#[derive(Clone, Serialize)]
pub struct Email {
	to: HashSet<EmailAddress>,
	subject: String,
	content: Content,
}

impl Email {
	pub fn builder() -> Self {
		Self {
			to: HashSet::new(),
			subject: "".to_string(),
			content: Content::builder(),
		}
	}

	pub fn to(&mut self, email_address: EmailAddress) -> &mut Self {
		self.to.insert(email_address);
		self
	}

	pub fn to_many(&mut self, email_addresses: Vec<EmailAddress>) -> &mut Self {
		for email_address in email_addresses.into_iter() {
			self.to.insert(email_address);
		}

		self
	}

	pub fn subject(&mut self, subject: &str) -> &mut Self {
		self.subject = subject.to_string();
		self
	}

	pub fn content(&mut self, content: Content) -> &mut Self {
		self.content = content;
		self
	}

	pub fn build(&self) -> Self {
		self.clone()
	}
}
