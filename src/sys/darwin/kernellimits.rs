// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::mem;
use std::ptr;

impl ::KernelLimits for ::HostInfo {
    #[allow(missing_docs)]
    fn max_file_handles(&self) -> i32 {
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
            panic!("max_file_handles: error calling sysctl");
        }
        maxfiles
    }
}
