// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::mem;
use std::ptr;
use os::SwapUsage;

#[allow(missing_docs)]
#[derive(Default)]
pub struct OperatingSystem {}

#[repr(C)]
#[derive(Default)]
struct XSwapUsage {
    xsu_total: u64,
    xsu_avail: u64,
    xsu_used: u64,
    xsu_pagesize: u32,
    xsu_encrypted: bool,
}

impl OperatingSystem {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(missing_docs)]
    pub fn boottime(&self) -> libc::timeval {
        let mut mib: [libc::c_int; 2] = [libc::CTL_KERN, libc::KERN_BOOTTIME];
        let mut boottime = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut size: libc::size_t = mem::size_of_val(&boottime);
        if unsafe {
               libc::sysctl(&mut mib[0],
                            2,
                            &mut boottime as *mut libc::timeval as *mut libc::c_void,
                            &mut size,
                            ptr::null_mut(),
                            0)
           } != 0 {
            panic!("boottime: error calling sysctl");
        }
        boottime
    }

    #[allow(missing_docs)]
    pub fn uptime(&self) -> i64 {
        let boot = self.boottime();
        let mut now = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        unsafe { libc::gettimeofday(&mut now, ptr::null_mut()) };
        now.tv_sec - boot.tv_sec
    }

    #[allow(missing_docs)]
    pub fn max_files(&self) -> i32 {
        let mut mib: [libc::c_int; 2] = [libc::CTL_KERN, libc::KERN_MAXFILESPERPROC];
        let mut maxfiles: libc::c_int = 0;
        let mut size: libc::size_t = mem::size_of_val(&maxfiles);
        if unsafe {
               libc::sysctl(&mut mib[0],
                            2,
                            &mut maxfiles as *mut libc::c_int as *mut libc::c_void,
                            &mut size,
                            ptr::null_mut(),
                            0)
           } != 0 {
            panic!("maxfiles: error calling sysctl");
        }
        maxfiles
    }

    #[allow(missing_docs)]
    pub fn swap_usage(&self) -> Option<SwapUsage> {
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
        Some(SwapUsage {
                 total: swap.xsu_total,
                 available: swap.xsu_avail,
                 used: swap.xsu_used,
                 page_size: swap.xsu_pagesize,
                 encrypted: swap.xsu_encrypted,
             })
    }
}
