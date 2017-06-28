// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use ::sys::linux::helpers;

impl ::SwapInfo for ::HostInfo {
    fn swap_usage(&self) -> Option<::SwapUsage> {
        let si = helpers::sysinfo();
        
        Some(::SwapUsage {
            total: si.totalswap * si.mem_unit as u64,
            available: si.freeswap * si.mem_unit as u64,
            used: si.totalswap * si.mem_unit as u64 - si.freeswap * si.mem_unit as u64,
            page_size: unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u32 },
            encrypted: false, // No reliable way to determine this for Linux
        })
    }
}
