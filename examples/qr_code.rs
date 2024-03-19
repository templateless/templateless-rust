use templateless::{
	components::{Image, Service, SocialItem, StoreBadge, StoreBadgeItem},
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

	let app_store_link = "https://apps.apple.com/us/app/example/id1234567890";
	let google_play_link =
		"https://play.google.com/store/apps/details?id=com.example";

	let footer = Footer::builder()
		.store_badges(&[
			StoreBadgeItem::new(StoreBadge::AppStore, app_store_link),
			StoreBadgeItem::new(StoreBadge::GooglePlay, google_play_link),
		])
		.socials(&[
			SocialItem::new(Service::Twitter, "MyCo"),
			SocialItem::new(Service::GitHub, "MyCo"),
		])
		.build()?;

	let content = Content::builder()
        .theme(Theme::Simple)
		.header(header)
		.text("Hey Alex,")
		.text("Thank you for choosing MyCo! To get started with our mobile experience, simply pair your account with our mobile app.")
        .text("Here's how to do it:")
		.text(&[
            &format!("1. Download the MyCo app from the [App Store]({app_store_link}) or [Google Play]({google_play_link})."),
            "1. Open the app and select _Pair Device_.",
            "1. Scan the QR code below with your mobile device:",
        ].join("\n"))
        .qr_code("https://example.com/qr-code-link")
		.text("Enjoy your seamless experience across devices!")
		.footer(footer)
		.build()?;

	let email = Email::builder()
		.to(EmailAddress::new(&email_address))
		.subject("How to Pair Device")
		.content(content)
		.build()?;

	let templateless = Templateless::new(&api_key);
	let _ = templateless.send(email).await?;

	Ok(())
}
