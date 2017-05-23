// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use libc::c_void;
use std::mem;

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, CFTypeRef, TCFType};
use core_foundation::boolean::{CFBooleanRef, kCFBooleanTrue};
use core_foundation::dictionary::{CFDictionaryRef, CFDictionary};
use core_foundation::number::{CFNumberRef, CFNumber};
use core_foundation::string::{CFStringRef, CFString};

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    pub fn IOPSCopyPowerSourcesInfo() -> CFTypeRef;
    pub fn IOPSCopyPowerSourcesList(blob: CFTypeRef) -> CFArrayRef;
    pub fn IOPSGetPowerSourceDescription(blob: CFTypeRef, ps: CFTypeRef) -> CFDictionaryRef;
    pub fn IOPSGetProvidingPowerSourceType(snapshot: CFTypeRef) -> CFStringRef;
}

const kIOPMUPSPowerKey: &'static str = "UPS Power";
const kIOPMBatteryPowerKey: &'static str = "Battery Power";
const kIOPMACPowerKey: &'static str = "AC Power";

const kIOPSCurrentCapacityKey: &'static str = "Current Capacity";
const kIOPSInternalType: &'static str = "Internal";
const kIOPSIsChargingKey: &'static str = "Is Charging";
const kIOPSIsPresentKey: &'static str = "Is Present";
const kIOPSMaxCapacityKey: &'static str = "Max Capacity";
const kIOPSTransportTypeKey: &'static str = "Transport Type";

struct PowerSourceDescription {
    data: CFDictionary,
}

impl PowerSourceDescription {
    fn new(blob: CFTypeRef, source: CFTypeRef) -> Self {
        let d: CFDictionary = unsafe {
            let d = IOPSGetPowerSourceDescription(blob, source);
            TCFType::wrap_under_get_rule(d)
        };
        PowerSourceDescription { data: d }
    }

    fn get_raw(&self, key: &'static str) -> *const c_void {
        let key = CFString::from_static_string(key);
        self.data.get(key.as_CFTypeRef())
    }

    fn get_cfstring(&self, key: &'static str) -> CFString {
        unsafe {
            let val: CFStringRef = mem::transmute(self.get_raw(key));
            TCFType::wrap_under_get_rule(val)
        }
    }

    fn get_bool(&self, key: &'static str) -> bool {
        unsafe {
            let val: CFBooleanRef = mem::transmute(self.get_raw(key));
            val == kCFBooleanTrue
        }
    }

    fn get_cfnumber(&self, key: &'static str) -> CFNumber {
        unsafe {
            let val: CFNumberRef = mem::transmute(self.get_raw(key));
            TCFType::wrap_under_get_rule(val)
        }
    }

    fn get_f64(&self, key: &'static str) -> Option<f64> {
        self.get_cfnumber(key).to_f64()
    }

    fn is_present(&self) -> bool {
        self.get_bool(kIOPSIsPresentKey)
    }

    fn is_charging(&self) -> bool {
        self.get_bool(kIOPSIsChargingKey)
    }
}

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

    fn battery_info(&self) -> Option<::BatteryInfo> {
        let info: CFType = unsafe {
            let info = IOPSCopyPowerSourcesInfo();
            TCFType::wrap_under_create_rule(info)
        };
        let sources: CFArray = unsafe {
            let sources = IOPSCopyPowerSourcesList(info.as_CFTypeRef());
            TCFType::wrap_under_create_rule(sources)
        };
        // Wrap the CFDictionary sources into a `PowerSourceDescription` so
        // that we can have handy helper methods. Then, check that it is an
        // internal battery and that it it present (not an empty battery slot).
        let bi = sources.iter()
            .map(|source| PowerSourceDescription::new(info.as_CFTypeRef(), source))
            .filter(|source| {
                        let transport_type = source.get_cfstring(kIOPSTransportTypeKey);
                        transport_type.to_string() == kIOPSInternalType
                    })
            .filter(|source| source.is_present())
            .take(1)
            .map(|source| {
                let charging = source.is_charging();
                let current = source.get_f64(kIOPSCurrentCapacityKey).expect("");
                let max = source.get_f64(kIOPSMaxCapacityKey).expect("");
                Some(::BatteryInfo {
                         charging: charging,
                         level: current / max,
                     })
            })
            .collect::<Vec<_>>();
        bi[0]
    }
}
