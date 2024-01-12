use dotenvy::dotenv;

use emailwrapper::{Email, EmailAddress, EmailWrapper, Template};

#[tokio::main]
async fn main() -> Result<(), emailwrapper::Error> {
	dotenv().ok();

	let api_key = match dotenvy::var("EMAILWRAPPER_API_KEY") {
		Ok(api_key) if !api_key.is_empty() => api_key,
		_ => {
			println!("No API key found in EMAILWRAPPER_API_KEY");
			return Ok(());
		}
	};

	let template = Template::builder().text("Hello world");

	let email = Email::builder()
		.to(EmailAddress::new("support@emailwrapper.com"))
		.subject("Hello")
		.template(template)
		.build();

	let emailwrapper = EmailWrapper::new(&api_key);
	let result = emailwrapper.send(email).await?;

	println!("{:?}", result);
	Ok(())
}
