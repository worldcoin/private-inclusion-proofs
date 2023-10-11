#[cfg(not(feature = "sgx"))]
mod non_sgx;
#[cfg(not(feature = "sgx"))]
pub use non_sgx::*;
