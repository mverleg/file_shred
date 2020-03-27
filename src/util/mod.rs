pub use errors::FedResult;

#[allow(clippy::module_inception)]
pub mod base64;
pub mod errors;
pub mod pth;
pub mod version;

#[cfg(test)]
pub mod test_util;