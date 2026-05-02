pub mod endpoint;
pub mod error;
pub mod jwt;
pub mod middleware;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
