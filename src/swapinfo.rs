// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Swap usage
///
/// This is obtained by calling `swap_usage` on a `HostInfo`:
///
/// ```rust
/// use hostinfo::{HostInfo, SwapInfo};
///
/// let hi = HostInfo::new();
/// if let Some(usage) = hi.swap_usage() {
///     assert!(usage.used <= usage.total);
/// }
/// ```
#[derive(Debug, Default)]
pub struct SwapUsage {
    /// Space, in bytes, allocated to swap space.
    pub total: u64,
    /// Space, in bytes, available for use within the swap space.
    pub available: u64,
    /// Space, in bytes, that has been used within the swap space.
    pub used: u64,
    /// Page size, in bytes, for the swap implementation.
    pub page_size: u32,
    /// Whether or not the swap space is encrypted.
    pub encrypted: bool,
}

/// Information about swap usage.
pub trait SwapInfo {
    /// Get information about swap usage. If swap is not
    /// present or enabled, then this will return `None`.
    ///
    /// ```rust
    /// use hostinfo::{HostInfo, SwapInfo};
    ///
    /// let hi = HostInfo::new();
    /// if let Some(usage) = hi.swap_usage() {
    ///     assert!(usage.available <= usage.total);
    /// }
    /// ```
    fn swap_usage(&self) -> Option<SwapUsage>;
}
