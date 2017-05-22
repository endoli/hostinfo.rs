// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Host Info
//!
//! This crate makes available information about the computer,
//! operating system and software that is running.
//!
//! All of the functionality is presented by traits which are
//! implemented by a `HostInfo` struct.
//!
//! ```rust
//! use hostinfo::{HostInfo, UptimeInfo};
//!
//! let hi = HostInfo::new();
//! let uptime = hi.uptime();
//! ```
#![warn(missing_docs)]
#![deny(trivial_numeric_casts, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate core_foundation;
extern crate libc;

mod sys;
pub use self::sys::HostInfo;

mod hardwareinfo;
pub use self::hardwareinfo::HardwareInfo;

mod kernellimits;
pub use self::kernellimits::KernelLimits;

mod powerinfo;
pub use self::powerinfo::{PowerInfo, PowerSource};

mod swapinfo;
pub use self::swapinfo::{SwapInfo, SwapUsage};

mod uptimeinfo;
pub use self::uptimeinfo::UptimeInfo;
