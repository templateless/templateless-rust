use serde::Serialize;

#[derive(Hash, PartialEq, Eq, Clone, Serialize)]
pub struct EmailAddress {
	name: Option<String>,
	email: String,
}

impl EmailAddress {
	pub fn new(email: &str) -> Self {
		Self { name: None, email: email.to_string() }
	}

	pub fn new_with_name(name: &str, email: &str) -> Self {
		Self { name: Some(name.to_string()), email: email.to_string() }
	}
}
