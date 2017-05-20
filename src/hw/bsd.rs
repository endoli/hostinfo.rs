// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::mem;
use std::ptr;

#[allow(missing_docs)]
#[derive(Default)]
pub struct Hardware {}

impl Hardware {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Default::default()
    }

    #[allow(missing_docs)]
    pub fn memory_size(&self) -> u64 {
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
