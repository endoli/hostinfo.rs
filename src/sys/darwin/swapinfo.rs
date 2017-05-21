// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::mem;
use std::ptr;

/// This struct is a representation of `struct xsw_usage` from
/// `sys/sysctl.h` on macOS.
#[repr(C)]
#[derive(Default)]
struct XSwapUsage {
    xsu_total: u64,
    xsu_avail: u64,
    xsu_used: u64,
    xsu_pagesize: u32,
    xsu_encrypted: bool,
}

impl ::SwapInfo for ::HostInfo {
    fn swap_usage(&self) -> Option<::SwapUsage> {
        let mut mib: [libc::c_int; 2] = [libc::CTL_VM, libc::VM_SWAPUSAGE];
        let mut swap = XSwapUsage::default();
        let mut size: libc::size_t = mem::size_of_val(&swap);
        if unsafe {
               libc::sysctl(&mut mib[0],
                            2,
                            &mut swap as *mut XSwapUsage as *mut libc::c_void,
                            &mut size,
                            ptr::null_mut(),
                            0)
           } != 0 {
            panic!("swap_usage: error calling sysctl");
        }
        Some(::SwapUsage {
                 total: swap.xsu_total,
                 available: swap.xsu_avail,
                 used: swap.xsu_used,
                 page_size: swap.xsu_pagesize,
                 encrypted: swap.xsu_encrypted,
             })
    }
}
