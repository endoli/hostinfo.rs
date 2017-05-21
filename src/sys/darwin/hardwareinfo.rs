// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::mem;
use std::ptr;

impl ::HardwareInfo for ::HostInfo {
    fn online_cpu_count(&self) -> u32 {
        let cpu_count = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
        if cpu_count == -1 {
            panic!("online_cpu_count: error calling sysconf");
        }
        cpu_count as u32
    }

    fn memory_size(&self) -> u64 {
        let mut mib: [libc::c_int; 2] = [libc::CTL_HW, libc::HW_MEMSIZE];
        let mut memsize: libc::uint64_t = 0;
        let mut size: libc::size_t = mem::size_of_val(&memsize);
        if unsafe {
               libc::sysctl(&mut mib[0],
                            2,
                            &mut memsize as *mut libc::uint64_t as *mut libc::c_void,
                            &mut size,
                            ptr::null_mut(),
                            0)
           } != 0 {
            panic!("memsize: error calling sysctl");
        }
        memsize
    }
}
