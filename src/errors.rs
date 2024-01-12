use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailWrapperError {
	#[error("unauthorized: invalid api key")]
	Unauthorized,
	#[error("unknown error")]
	Unknown,
}
