use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplatelessError {
	#[error("service unavailable")]
	Unavailable,
	#[error("unauthorized: invalid api key")]
	Unauthorized,
	#[error("unknown error")]
	Unknown,
}
