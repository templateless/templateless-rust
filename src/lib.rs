pub use email::Email;
pub use email_address::EmailAddress;
pub use email_wrapper::EmailWrapper;
pub use errors::EmailWrapperError;
pub use template::Template;

static EMAILWRAPPER_DOMAIN: &str = "https://emailwrapper.com";

pub type Error = EmailWrapperError;
pub type EmailWrapperResult<T> = Result<T, Error>;

pub type ObjectId = String;

pub mod components;
mod email;
mod email_address;
mod email_wrapper;
mod errors;
mod template;
