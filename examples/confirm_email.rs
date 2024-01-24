use templateless::{
	utils, Content, Email, EmailAddress, Footer, Header, Result, Service,
	SocialItem, Templateless,
};

#[tokio::main]
async fn main() -> Result<()> {
	let api_key = utils::get_env("TEMPLATELESS_API_KEY");
	let email_address = utils::get_env("TEMPLATELESS_EMAIL_ADDRESS");

	let header = Header::builder().text("# ExampleApp").build()?;

	let footer = Footer::builder()
		.socials(vec![
			SocialItem::new(Service::Twitter, "ExampleApp"),
			SocialItem::new(Service::Github, "ExampleApp"),
		])
		.link("Unsubscribe", "https://example.com/unsubscribe?id=123")
		.build()?;

	let content = Content::builder()
		.header(header)
		.text("Hello world")
		.footer(footer)
		.build()?;

	let email = Email::builder()
		.to(EmailAddress::new(&email_address))
		.subject("Confirm your email")
		.content(content)
		.build()?;

	let templateless = Templateless::new(&api_key);
	let result = templateless.send(email).await?;

	println!("{:?}", result);
	Ok(())
}
