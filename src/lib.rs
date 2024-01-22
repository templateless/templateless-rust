pub use collection::{Collection as Header, Collection as Footer};
pub use components::{ListStyle, Service, SocialItem};
pub use content::{Content, Theme};
pub use email::Email;
pub use email_address::EmailAddress;
pub use email_wrapper::Templateless;
pub use errors::TemplatelessError;

static TEMPLATELESS_DOMAIN: &str = "https://templateless.com";

pub type Error = TemplatelessError;
pub type TemplatelessResult<T> = Result<T, Error>;

pub type ObjectId = String;

mod collection;
pub mod components;
mod content;
mod email;
mod email_address;
mod email_wrapper;
mod errors;
pub mod utils;
