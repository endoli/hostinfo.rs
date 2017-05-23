// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Basic battery info.
///
/// This is obtained by calling `battery_info` on a `HostInfo`:
///
/// ```rust
/// use hostinfo::{HostInfo, PowerInfo};
///
/// let hi = HostInfo::new();
/// let bi = hi.battery_info();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BatteryInfo {
    /// Is the battery currently charging?
    pub charging: bool,
    /// The current capacity of the battery, as a percentage,
    /// scaled to a value between `0.0` and `1.0`.
    pub level: f64,
}

/// What is the power source for the machine?
///
/// This is obtained by calling `power_source` on a `HostInfo`:
///
/// ```rust
/// use hostinfo::{HostInfo, PowerInfo};
///
/// let hi = HostInfo::new();
/// let ps = hi.power_source();
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
    /// let ps = hi.power_source();
    /// ```
    fn power_source(&self) -> PowerSource;

    /// Get basic info about current battery status.
    ///
    /// ```rust
    /// use hostinfo::{HostInfo, PowerInfo};
    ///
    /// let hi = HostInfo::new();
    /// let ps = hi.battery_info();
    /// ```
    fn battery_info(&self) -> Option<BatteryInfo>;
}
