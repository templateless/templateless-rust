use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error, Deserialize)]
pub enum BadRequestCode {
	#[error("account quota exceeded")]
	AccountQuotaExceeded = 200,
	#[error("provider key missing")]
	ProviderKeyMissing = 300,
	#[error("provider key invalid")]
	ProviderKeyInvalid = 301,
}

#[derive(Debug, Error, Deserialize)]
pub enum TemplatelessError {
	#[error("unauthorized")]
	Unauthorized,
	#[error("forbidden")]
	Forbidden,
	#[error("invalid parameter")]
	InvalidParameter { field: String },
	#[error("bad request")]
	BadRequest { code: BadRequestCode, error: String },
	#[error("unavailable")]
	Unavailable,
	#[error("unknown error")]
	Unknown,
}
