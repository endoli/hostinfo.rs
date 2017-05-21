// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Limits imposed by the operating system kernel
pub trait KernelLimits {
    /// Maximum number of file handles that may be created by
    /// a process.
    ///
    /// This is distinct from the limits set by the `ulimit`
    /// command. This value is the upper limit on what is allowed.
    ///
    /// ```rust
    /// use hostinfo::{HostInfo, KernelLimits};
    ///
    /// let hi = HostInfo::new();
    /// assert!(hi.max_file_handles() > 0);
    /// ```
    fn max_file_handles(&self) -> i32;
}
