use emailwrapper::{utils, Email, EmailAddress, EmailWrapper, Template};

#[tokio::main]
async fn main() -> Result<(), emailwrapper::Error> {
	let api_key = utils::get_env("EMAILWRAPPER_API_KEY");
	let email_address = utils::get_env("EMAILWRAPPER_EMAIL_ADDRESS");

	let template = Template::builder().text("Hello world");

	let email = Email::builder()
		.to(EmailAddress::new(&email_address))
		.subject("Hello")
		.template(template)
		.build();

	let emailwrapper = EmailWrapper::new(&api_key);
	let result = emailwrapper.send(email).await?;

	println!("{:?}", result);
	Ok(())
}
