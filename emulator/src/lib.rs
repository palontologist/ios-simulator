//! iOSS Emulator - Open-source iOS Simulator for NixOS
//!
//! This library provides the core emulator functionality for running iOS apps
//! on NixOS without relying on proprietary Apple frameworks or kernels.

pub mod device;
pub mod emulator;
pub mod frameworks;
pub mod runtime;
pub mod services;
pub mod error;

pub use device::Device;
pub use emulator::Emulator;
pub use error::{Error, Result};

/// iOSS version
pub const VERSION: &str = "0.1.0";

/// Supported iOS versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum iOSVersion {
    /// iOS 14.x
    iOS14,
    /// iOS 15.x
    iOS15,
}

impl iOSVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            iOSVersion::iOS14 => "14.0",
            iOSVersion::iOS15 => "15.0",
        }
    }
}

impl std::fmt::Display for iOSVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "iOS {}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ios_version_display() {
        assert_eq!(iOSVersion::iOS15.to_string(), "iOS 15.0");
    }
}
