// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod hardwareinfo;
mod kernellimits;
mod powerinfo;
mod swapinfo;
mod uptimeinfo;

mod helpers;

/// Entry point for accessing information about this host.
#[derive(Default)]
pub struct HostInfo {}

impl HostInfo {
    /// Create a new instance of `HostInfo`.
    pub fn new() -> Self {
        Default::default()
    }
}
