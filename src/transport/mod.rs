pub mod slip;
pub mod serial;
pub mod h5;

use crate::{Error, Result};

pub trait Transport {
    fn open() -> Result<()>;
    fn close();
}