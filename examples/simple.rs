use templateless::{utils, Content, Email, EmailAddress, Result, Templateless};

#[tokio::main]
async fn main() -> Result<()> {
	let api_key = utils::get_env("TEMPLATELESS_API_KEY");
	let email_address = utils::get_env("TEMPLATELESS_EMAIL_ADDRESS");

	Templateless::new(&api_key)
		.send(
			Email::builder()
				.to(EmailAddress::new(&email_address))
				.subject("Hello 👋")
				.content(Content::builder().text("Hello world").build()?)
				.build()?,
		)
		.await?;

	Ok(())
}
