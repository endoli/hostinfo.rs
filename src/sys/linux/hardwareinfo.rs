// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::File;
use std::io::Read;
use ::sys::linux::helpers;

impl ::HardwareInfo for ::HostInfo {
    fn online_cpu_count(&self) -> u32 {
        let mut file = File::open("/sys/devices/system/cpu/online").unwrap();
        let mut online_cpu_ranges = String::new();
        file.read_to_string(&mut online_cpu_ranges).unwrap();

        let mut cpu_count = 0u32;

        for range in online_cpu_ranges.split(',') {
            cpu_count += match range.find('-') {
                Some(idx) => {
                    let range = range.split_at(idx);
                    let start = range.0.parse::<u32>().unwrap();
                    let stop = range.1[1..].trim().parse::<u32>().unwrap();
                    stop - start + 1
                },
                None => 1,
            }
        };
        
        cpu_count
    }

    fn memory_size(&self) -> u64 {
        let si = helpers::sysinfo();
        si.totalram
    }
}
