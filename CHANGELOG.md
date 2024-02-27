# CHANGELOG

**v0.1.0-alpha.6:**
- `README.md` cleanup (listing of components)
- `Image` component now requires only `src`; the other params are optional
  - `content.image(...)` param order has changed (2nd param is `url`, not `alt`)
- `ViewInBrowser` component has changed: text is optional

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