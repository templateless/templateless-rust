use templateless::{
	components::{Image, Service, SocialItem},
	utils, Content, Email, EmailAddress, Footer, Header, Result, Templateless,
	Theme,
};

#[tokio::main]
async fn main() -> Result<()> {
	let api_key = utils::get_env("TEMPLATELESS_API_KEY");
	let email_address = utils::get_env("TEMPLATELESS_EMAIL_ADDRESS");

	let header = Header::builder()
		.component(
			Image::new("https://templateless.net/myco.webp")
				.width(100)
				.alt("MyCo")
				.build()?,
		)
		.build()?;

	let footer = Footer::builder()
	.text("If you did not sign up for a MyCo account, please ignore this email.\nThis link will expire in 24 hours.")
		.socials(&[
			SocialItem::new(Service::Twitter, "MyCo"),
			SocialItem::new(Service::GitHub, "MyCo"),
		])
		.link("Unsubscribe", "https://example.com")
		.build()?;

	let verify_email_link = "https://example.com/verify?token=ABC";

	let content = Content::builder()
        .theme(Theme::Simple)
		.header(header)
		.text("Hi there,")
		.text("Welcome to **MyCo**! We're excited to have you on board. Before we get started, we need to verify your email address.")
        .text("Please confirm your email by clicking the button below:")
		.button("Verify Email", verify_email_link)
		.text("Or use the link below:")
		.link(verify_email_link, verify_email_link)
		.footer(footer)
		.build()?;

	let email = Email::builder()
		.to(EmailAddress::new(&email_address))
		.subject("Confirm your email")
		.content(content)
		.build()?;

	let templateless = Templateless::new(&api_key);
	let _ = templateless.send(email).await?;

	Ok(())
}
