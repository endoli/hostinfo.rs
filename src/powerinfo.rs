// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// What is the power source for the machine?
///
/// This is obtained by calling `power_source` on a `HostInfo`:
///
/// ```rust
/// use hostinfo::{HostInfo, PowerInfo};
///
/// let hi = HostInfo::new();
/// let ps = hi.power_source() {
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerSource {
    /// An external unlimited power source.
    AC,
    /// An internal battery power source.
    Battery,
    /// An external power source, a UPS.
    UPS,
    /// An unknown power source. (This should never happen.)
    Unknown,
}

/// Information about how the machine is powered.
pub trait PowerInfo {
    /// What source is the computer currently powered by.
    ///
    /// ```rust
    /// use hostinfo::{HostInfo, PowerInfo};
    ///
    /// let hi = HostInfo::new();
    /// let ps = hi.power_source() {
    /// ```
    fn power_source(&self) -> PowerSource;
}
