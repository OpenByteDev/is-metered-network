//! Cross-platform detection of whether the current network connection is metered.
//!
//! A metered connection is one where the operating system (or the user) has indicated that
//! data usage may be limited or billed, e.g. cellular data, a mobile hotspot or a capped plan.
//! Applications can use this to avoid large background transfers on such connections.
//!
//! | Platform | Source |
//! | -------- | ------ |
//! | Windows | the Network List Manager COM API ([`INetworkCostManager::GetCost`](https://learn.microsoft.com/en-us/windows/win32/api/netlistmgr/nf-netlistmgr-inetworkcostmanager-getcost)) |
//! | Linux | the [`Metered`](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html) property of `org.freedesktop.NetworkManager` on the D-Bus system bus |
//!
//! macOS is not supported yet.
//!
//! ## Usage
//! ```rust
//! # fn main() -> std::io::Result<()> {
//! if is_metered_network::check()? {
//!     println!("on a metered connection");
//! }
//! # Ok(())
//! # }
//! ```

#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::io;

#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(not(any(windows, target_os = "linux")))]
compile_error!("is-metered-network only supports Windows and Linux (macOS is not supported yet)");

/// The cost/metered status of a network connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionCost {
    /// The connection is unrestricted (e.g. most Wi-Fi and Ethernet connections).
    Unmetered,
    /// The connection is metered (e.g. cellular data, a mobile hotspot, or a capped/pay-per-byte plan).
    Metered,
    /// We could not determine the cost of the connection.
    Unknown,
}

impl ConnectionCost {
    /// Returns `true` if the connection is known to be [`Metered`](ConnectionCost::Metered).
    ///
    /// [`Unknown`](ConnectionCost::Unknown) is treated as not metered.
    #[must_use]
    pub fn is_metered(self) -> bool {
        matches!(self, Self::Metered)
    }
}

/// Returns the cost/metered status of the current primary network connection.
///
/// # Errors
/// Returns an error if the underlying platform API could not be queried at all
/// (as opposed to it reporting an unknown cost, which is [`Ok(ConnectionCost::Unknown)`](ConnectionCost::Unknown)).
pub fn connection_cost() -> io::Result<ConnectionCost> {
    #[cfg(windows)]
    return windows::connection_cost();

    #[cfg(target_os = "linux")]
    return linux::connection_cost();
}

/// Returns `true` if the current network connection is metered.
///
/// # Errors
/// See [`connection_cost`].
pub fn check() -> io::Result<bool> {
    connection_cost().map(ConnectionCost::is_metered)
}
