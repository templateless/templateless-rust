# [EmailWrapper](https://emailwrapper.com) Rust

[![Github Actions](https://img.shields.io/github/actions/workflow/status/emailwrapper/emailwrapper-rust/tests.yml?style=flat-square)](https://github.com/emailwrapper/emailwrapper-rust/actions)
[![License](https://img.shields.io/github/license/emailwrapper/emailwrapper-rust?color=green&style=flat-square)](/LICENSE)

## What is this?

[EmailWrapper](https://emailwrapper.com) lets you quickly create and send emails with your favorite email provider without ever leaving your code editor.

Don't waste time messing around with HTML or HTML builders.

**Get your free API key [here](https://emailwrapper.com).**

## Quick Example

```rust
use emailwrapper::{Email, EmailAddress, EmailWrapper, Template};

#[tokio::main]
async fn main() -> Result<(), emailwrapper::Error> {
    let template = Template::builder()
        .text("Hello world");

    let email = Email::builder()
        .to(EmailAddress::new("user@example.com"))
        .subject("Hello")
        .template(template);

    let emailwrapper = EmailWrapper::new("<YOUR_API_KEY>");
    let _result = emailwrapper.send(email).await?;

    Ok(())
}
```

## More Examples

1. Test a very simple email:

    ```shell
    EMAILWRAPPER_API_KEY=<YOUR_API_KEY> \
      EMAILWRAPPER_EMAIL_ADDRESS=<YOUR_EMAIL_ADDRESS> \
      cargo run --example simple
    ```

1. Sample "confirm email" when user signs up:

    ```shell
    EMAILWRAPPER_API_KEY=<YOUR_API_KEY> \
      EMAILWRAPPER_EMAIL_ADDRESS=<YOUR_EMAIL_ADDRESS> \
      cargo run --example confirm-email
    ```

## License

[MIT](LICENSE)