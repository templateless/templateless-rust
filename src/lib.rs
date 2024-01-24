pub use collection::{Collection as Header, Collection as Footer};
pub use components::{Service, SocialItem};
pub use content::{Content, Theme};
pub use email::Email;
pub use email_address::EmailAddress;
pub use errors::{Error, Result};
pub use templateless::Templateless;

static TEMPLATELESS_DOMAIN: &str = "https://templateless.com";

pub type ObjectId = String;

mod collection;
pub mod components;
mod content;
mod email;
mod email_address;
mod errors;
mod templateless;
pub mod utils;
