// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;

/// How long has the operating system been running?
pub trait UptimeInfo {
    /// Time at which the operating system booted.
    ///
    /// TODO: It would be nice if this used another type for the time,
    /// perhaps `chrono` or another crate?
    ///
    /// ```rust
    /// use hostinfo::{HostInfo, UptimeInfo};
    ///
    /// let hi = HostInfo::new();
    /// let booted = hi.boottime();
    /// assert!(booted.tv_sec > 0);
    /// ```
    fn boottime(&self) -> libc::timeval;

    /// Time, in seconds, since the operating system booted.
    ///
    /// ```rust
    /// use hostinfo::{HostInfo, UptimeInfo};
    ///
    /// let hi = HostInfo::new();
    /// let uptime = hi.uptime();
    /// assert!(uptime > 0);
    /// ```
    fn uptime(&self) -> i64;
}
