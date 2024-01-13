use emailwrapper::{
	utils, Email, EmailAddress, EmailWrapper, Footer, Header, Service,
	SocialItem, Template,
};

#[tokio::main]
async fn main() -> Result<(), emailwrapper::Error> {
	let api_key = utils::get_env("EMAILWRAPPER_API_KEY");
	let email_address = utils::get_env("EMAILWRAPPER_EMAIL_ADDRESS");

	let header = Header::builder().text("# ExampleApp").build();

	let footer = Footer::builder()
		.socials(vec![
			SocialItem::new(Service::Twitter, "ExampleApp"),
			SocialItem::new(Service::Github, "ExampleApp"),
		])
		.link("Unsubscribe", "https://example.com/unsubscribe?id=123")
		.build();

	let template = Template::builder()
		.header(header)
		.text("Hello world")
		.footer(footer)
		.build();

	let email = Email::builder()
		.to(EmailAddress::new(&email_address))
		.subject("Confirm your email")
		.template(template)
		.build();

	let emailwrapper = EmailWrapper::new(&api_key);
	let result = emailwrapper.send(email).await?;

	println!("{:?}", result);
	Ok(())
}
