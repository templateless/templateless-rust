# CHANGELOG

## v0.1.0-alpha.8:

### Breaking Changes
- Function definition for `.socials()` accepts array instead of vector now
- `Service`, `SocialItem` have been moved to `components`
- Function definition for `.image()` has changed:

  ```rust
  // Old ❌
  let content = Content::builder()
    .image(
      "https://example.com/image.jpg",
      Some("https://example.com"),
      Some(200), // Width
      Some(100), // Height
      Some("Alt Text"),
    )
    .build()?;
  
  // New (just image) ✅
  let content = Content::builder()
    .image("https://example.com/image.jpg")
    .build()?;
  
  // New (clickable image with custom attributes) ✅
  let content = Content::builder()
    .component(
      Image::new("https://example.com/image.jpg")
        .url("https://example.com")
        .width(200)
        .height(100)
        .alt("Alt Text")
        .build()?
    )
    .build()?;
  ```

- Function definition for `.view_in_browser()` has changed:

  ```rust
  // Old ❌
  let content = Content::builder()
    .view_in_browser(Some("Read Online".to_string()))
    .build()?;
  
  // New (default text "View in browser") ✅
  let content = Content::builder()
    .view_in_browser()
    .build()?;
  
  // New (custom text) ✅
  let content = Content::builder()
    .component(ViewInBrowser::new("Read Online"))
    .build()?;
  ```

### New Features

- New social icons: `Service::Mastodon` and `Service::Rss`
- New store badges component
- New QR code component
- New signature component

## v0.1.0-alpha.7:
- `README.md`
  - markdown supports ordered/unordered lists
  - notice about test mode
- Support for test mode logging

## v0.1.0-alpha.6:
- `README.md` cleanup (listing of components)
- `Image` component now requires only `src`; the other params are optional
  - `content.image(...)` param order has changed (2nd param is `url`, not `alt`)
- `ViewInBrowser` component has changed: text is optional

## v0.1.0-alpha.5:
- Introduced `CHANGELOG.md`
- Introduced new services as social icons:
  - `Service::Phone` (converts into a link with `tel:` prefix)
  - `Service::Facebook`
  - `Service::YouTube`
  - `Service::Instagram`
  - `Service::LinkedIn`
  - `Service::Slack`
  - `Service::Discord`
  - `Service::TikTok`
  - `Service::Snapchat`
  - `Service::Threads`
  - `Service::Telegram`
- **Breaking Changes:**
  - Renamed `Service::Github` to `Service::GitHub`

## v0.1.0-alpha.4:
- Initial implementation