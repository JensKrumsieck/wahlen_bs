mod error;
mod models;

pub mod http;
pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;
