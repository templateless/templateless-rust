use serde::Deserialize;
use std::result;
use thiserror::Error;

#[derive(Debug, Error, Deserialize, Eq, PartialEq)]
pub enum BadRequestCode {
	#[error("account quota exceeded")]
	AccountQuotaExceeded = 200,
	#[error("provider key missing")]
	ProviderKeyMissing = 300,
	#[error("provider key invalid")]
	ProviderKeyInvalid = 301,
	#[error("email has no content to send")]
	EmailNoContent = 400,
}

#[derive(Debug, Error, Deserialize, Eq, PartialEq)]
pub enum Error {
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

pub type Result<T> = result::Result<T, Error>;
