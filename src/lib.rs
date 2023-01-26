
//mod sd_api_v6;
mod error;
pub mod transport;


//pub use sd_api_v6::*;
pub use error::Error;


pub type Result<T> = std::result::Result<T, error::Error>;



