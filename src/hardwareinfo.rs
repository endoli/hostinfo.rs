// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Information about the hardware.
pub trait HardwareInfo {
    /// In bytes, how much physical memory is present.
    ///
    /// ```rust
    /// use hostinfo::{HardwareInfo, HostInfo};
    ///
    /// let hi = HostInfo::new();
    /// let m = hi.memory_size();
    /// assert!(m > 0);
    /// ```
    fn memory_size(&self) -> u64;
}
