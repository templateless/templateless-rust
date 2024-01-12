# EmailWrapper Rust

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

    let emailwrapper = EmailWrapper::new("<API_KEY>");
    let _result = emailwrapper.send(email).await?;

    Ok(())
}
```

## License

[MIT](LICENSE)