// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;

impl ::KernelLimits for ::HostInfo {
    fn max_file_handles(&self) -> i32 {
        unsafe {
            let mut limits = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
            libc::getrlimit(libc::RLIMIT_NOFILE, &mut limits);
            limits.rlim_cur as i32
        }
    }
}
