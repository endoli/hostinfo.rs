// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct SwapUsage {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub page_size: u32,
    pub encrypted: bool,
}

#[allow(missing_docs)]
pub trait SwapInfo {
    fn swap_usage(&self) -> Option<SwapUsage>;
}
