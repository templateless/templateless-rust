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
		.socials(&[
			SocialItem::new(Service::Twitter, "MyCo"),
			SocialItem::new(Service::GitHub, "MyCo"),
		])
		.build()?;

	let content = Content::builder()
        .theme(Theme::Simple)
		.header(header)
		.text("Hey Alex,")
		.text("I'm Jamie, founder of **MyCo**.")
        .text("Could you spare a moment to reply to this email with your thoughts on our service? Your feedback is invaluable and directly influences our improvements.")
		.text("When you hit reply, your email will go directly to me, and I read each and every one.")
		.text("Thanks for your support,")
        .signature("Jamie Parker")
		.text("Jamie Parker\n\nFounder @ [MyCo](https://example.com)")
		.footer(footer)
		.build()?;

	let email = Email::builder()
		.to(EmailAddress::new(&email_address))
		.subject("Thoughts on service?")
		.content(content)
		.build()?;

	let templateless = Templateless::new(&api_key);
	let _ = templateless.send(email).await?;

	Ok(())
}
