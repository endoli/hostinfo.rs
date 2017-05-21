// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::mem;
use std::ptr;

impl ::UptimeInfo for ::HostInfo {
    fn boottime(&self) -> libc::timeval {
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

    fn uptime(&self) -> i64 {
        let boot = self.boottime();
        let mut now = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        unsafe { libc::gettimeofday(&mut now, ptr::null_mut()) };
        now.tv_sec - boot.tv_sec
    }
}
