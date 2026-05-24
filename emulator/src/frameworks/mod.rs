//! iOS Framework stubs
//!
//! Provides simulation of iOS frameworks like UIKit, Foundation, etc.

pub mod foundation;
pub mod uikit;
pub mod network;

pub use foundation::Foundation;
pub use uikit::UIKit;
pub use network::Network;
