# CHANGELOG

**v0.1.0-alpha.6:**
- Image component now requires only `src`; the other params are optional
- `Image::new()` param order has changed (2nd param is `url`, not `alt`)

**v0.1.0-alpha.5:**
- Introduced `CHANGELOG.md`
- Introduced new services as [social icons](examples/confirm_email.rs):
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

**v0.1.0-alpha.4:**
- Initial implementation