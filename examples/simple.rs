use templateless::{utils, Content, Email, EmailAddress, Result, Templateless};

#[tokio::main]
async fn main() -> Result<()> {
	let api_key = utils::get_env("TEMPLATELESS_API_KEY");
	let email_address = utils::get_env("TEMPLATELESS_EMAIL_ADDRESS");

	let content = Content::builder().text("Hello world").build()?;

	let email = Email::builder()
		.to(EmailAddress::new(&email_address))
		.subject("Hello")
		.content(content)
		.build()?;

	let templateless = Templateless::new(&api_key);
	let result = templateless.send(email).await?;

	println!("{:?}", result);
	Ok(())
}
