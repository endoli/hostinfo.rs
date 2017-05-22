// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use core_foundation::base::{CFType, CFTypeRef, TCFType};
use core_foundation::string::{CFStringRef, CFString};

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    pub fn IOPSCopyPowerSourcesInfo() -> CFTypeRef;
    pub fn IOPSGetProvidingPowerSourceType(snapshot: CFTypeRef) -> CFStringRef;
}

const kIOPMUPSPowerKey: &'static str = "UPS Power";
const kIOPMBatteryPowerKey: &'static str = "Battery Power";
const kIOPMACPowerKey: &'static str = "AC Power";

impl ::PowerInfo for ::HostInfo {
    fn power_source(&self) -> ::PowerSource {
        let info: CFType = unsafe {
            let info = IOPSCopyPowerSourcesInfo();
            TCFType::wrap_under_create_rule(info)
        };
        let source_type: CFString = unsafe {
            let source_type = IOPSGetProvidingPowerSourceType(info.as_CFTypeRef());
            TCFType::wrap_under_get_rule(source_type)
        };
        let source_type = source_type.to_string();
        if source_type == kIOPMACPowerKey {
            ::PowerSource::AC
        } else if source_type == kIOPMBatteryPowerKey {
            ::PowerSource::Battery
        } else if source_type == kIOPMUPSPowerKey {
            ::PowerSource::UPS
        } else {
            ::PowerSource::Unknown
        }
    }
}
