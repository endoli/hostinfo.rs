// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod hardwareinfo;
mod kernellimits;
mod swapinfo;
mod uptimeinfo;

#[allow(missing_docs)]
#[derive(Default)]
pub struct HostInfo {}

impl HostInfo {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Default::default()
    }
}
