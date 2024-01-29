# Templateless Rust

[![Latest version](https://img.shields.io/crates/v/templateless.svg)](https://crates.io/crates/templateless)
[![Github Actions](https://img.shields.io/github/actions/workflow/status/templateless/templateless-rust/tests.yml)](https://github.com/templateless/templateless-rust/actions)
[![Docs](https://docs.rs/templateless/badge.svg)](https://docs.rs/templateless/latest/templateless/)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/templateless)](https://crates.io/crates/templateless)

## What is this?

[Templateless](https://templateless.com) lets you quickly create and send emails with your favorite email provider without ever leaving your code editor.

Don't waste time messing around with HTML or HTML builders.

**Get your free API key [here](https://app.templateless.com).**

## Quick Example

```rust
use templateless::{Content, Email, EmailAddress, Templateless, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let content = Content::builder()
        .text("Hello world")
        .build()?;

    let email = Email::builder()
        .to(EmailAddress::new("user@example.com"))
        .subject("Hello 👋")
        .content(content)
        .build()?;

    let _result = Templateless::new("<YOUR_API_KEY>")
        .send(email)
        .await?;

    Ok(())
}
```

## More Examples

1. Test a very simple email:

    ```bash
    TEMPLATELESS_API_KEY=<YOUR_API_KEY> \
      TEMPLATELESS_EMAIL_ADDRESS=<YOUR_EMAIL_ADDRESS> \
      cargo run --example simple
    ```

1. Sample "confirm email" when user signs up:

    ```bash
    TEMPLATELESS_API_KEY=<YOUR_API_KEY> \
      TEMPLATELESS_EMAIL_ADDRESS=<YOUR_EMAIL_ADDRESS> \
      cargo run --example confirm_email
    ```

## License

[MIT](LICENSE)