use serde::Serialize;
use std::collections::HashSet;

use crate::{EmailAddress, Template};

#[derive(Clone, Serialize)]
pub struct Email {
	to: HashSet<EmailAddress>,
	subject: String,
	template: Template,
}

impl Email {
	pub fn builder() -> Self {
		Self {
			to: HashSet::new(),
			subject: "".to_string(),
			template: Template::builder(),
		}
	}

	pub fn to(&mut self, email_address: EmailAddress) -> &mut Self {
		self.to.insert(email_address);
		self
	}

	pub fn subject(&mut self, subject: &str) -> &mut Self {
		self.subject = subject.to_string();
		self
	}

	pub fn template(&mut self, template: Template) -> &mut Self {
		self.template = template;
		self
	}

	pub fn build(&self) -> Self {
		self.clone()
	}
}
