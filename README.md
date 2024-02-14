<h1 align="center">
  <a href="https://templateless.com/">
    <img src="templateless.webp" alt="Templateless" width="450px">
  </a>
  <br />
</h1>

<p align="center">
  <b>Ship faster by treating email as code 🚀</b> <br />
</p>

<h4 align="center">
  <a href="https://templateless.com/">Website</a> &bull;
  <a href="https://app.templateless.com/">Get Your API Key</a> &bull;
  <a href="https://twitter.com/templateless">Twitter</a>
</h4>

---

[![Latest version](https://img.shields.io/crates/v/templateless.svg)](https://crates.io/crates/templateless)
[![Github Actions](https://img.shields.io/github/actions/workflow/status/templateless/templateless-rust/tests.yml)](https://github.com/templateless/templateless-rust/actions)
[![Docs](https://docs.rs/templateless/badge.svg)](https://docs.rs/templateless/latest/templateless/)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/templateless)](https://crates.io/crates/templateless)
[![X (formerly Twitter) Follow](https://img.shields.io/twitter/follow/templateless)](https://twitter.com/templateless)

[Templateless](https://templateless.com) lets you generate and send transactional emails quickly and easily so you can focus on building your product.

## ✨ Features

- 👋 **Anti drag-and-drop by design** — emails are a part of your code
- ✅ **Components as code** — function calls turn into email HTML components
- 💻 **SDK for any language** — use your favorite [programming language](https://github.com/orgs/templateless/repositories)
- 🔍 **Meticulously tested** — let us worry about email client compatibility
- 💌 **Use your favorite ESP** — Amazon SES, SendGrid, Mailgun + more
- 💪 **Email infrastructure** — rate-limiting, retries, scheduling + more
- ⚡ **Batch sending** — send 1 email or 1,000 with one API call

## 🚀 Getting started

Install via Cargo:

```bash
cargo add templateless
```

Or add manually to your `Cargo.toml`:

```toml
[dependencies]
templateless = "0.1"
```

## 👩‍💻 Quick example

This is all it takes to send a signup confirmation email:

```rust
use templateless::{Content, Email, EmailAddress, Templateless, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let content = Content::builder()
        .text("Hi, please **confirm your email**:")
        .button("Confirm Email", "https://your-company.com/signup/confirm?token=XYZ")
        .build()?;

    let email = Email::builder()
        .to(EmailAddress::new("<YOUR_CUSTOMERS_EMAIL_ADDRESS>"))
        .subject("Confirm your signup 👋")
        .content(content)
        .build()?;

    let _result = Templateless::new("<YOUR_API_KEY>")
        .send(email)
        .await?;

    Ok(())
}
```

> **Note**
> 🚧 **This SDK is not stable yet.** The API might change as more features are added. Please pay attention to the [CHANGELOG](CHANGELOG.md) for breaking changes.

Examples:

1. Get your **free API key** here: <https://app.templateless.com> ✨
1. There are more Rust examples in the [examples](examples) folder

## 🤝 Contributing

- Contributions are more than welcome <3
- Please **star this repo** for more visibility ★

## 📫 Get in touch

- For customer support feel free to email us at [github@templateless.com](mailto:github@templateless.com)

- Have suggestions or want to give feedback? Here's how to reach us:

    - For feature requests, please [start a discussion](https://github.com/templateless/templateless-rust/discussions)
    - Found a bug? [Open an issue!](https://github.com/templateless/templateless-rust/issues)
    - We are also on Twitter: [@Templateless](https://twitter.com/templateless)

## 🍻 License

[MIT](LICENSE)