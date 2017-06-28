// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::ptr;
use ::sys::linux::helpers;

impl ::UptimeInfo for ::HostInfo {
    fn boottime(&self) -> libc::timeval {
        let mut timestamp = libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        unsafe {
            libc::gettimeofday(&mut timestamp, ptr::null_mut());
        }
        timestamp.tv_usec = 0; // Can't know this part
        timestamp.tv_sec -= self.uptime();
        timestamp
    }

    fn uptime(&self) -> i64 {
        let si = helpers::sysinfo();
        si.uptime
    }
}
