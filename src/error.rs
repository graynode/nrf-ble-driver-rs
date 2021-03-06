

use std::ffi;

#[derive(Debug)]
pub enum Error {
    /// Error initializing the softdevice api
    InitializationError,

    FFIError(u32),
    
    NullError(ffi::NulError),
}

