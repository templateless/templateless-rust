use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailWrapperError {
	#[error("service unavailable")]
	Unavailable,
	#[error("unauthorized: invalid api key")]
	Unauthorized,
	#[error("unknown error")]
	Unknown,
}
