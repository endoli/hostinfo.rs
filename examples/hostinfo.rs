extern crate hostinfo;
use hostinfo::*;

fn main() {
    let hi = HostInfo::new();
    
    println!("\nHardware info:\n===============");
    println!(" Online cores: {}", hi.online_cpu_count());
    println!(" Total memory: {} MiB", hi.memory_size() / 1024 / 1024);
    println!("\nOperating system:\n=================");
    println!(" Maximum file handles per process: {}", hi.max_file_handles());
    println!(" Uptime: {} seconds", hi.uptime());
    match hi.swap_usage() {
        Some(su) => {
            println!("\nSwap memory:\n============");
            println!(" Total swap: {} MiB ({} MiB in use)", su.total / 1024 / 1024,
                     su.used / 1024 / 1024);
            println!(" Encrypted: {}", if su.encrypted { "yes" } else { "no" });
            println!(" Page size: {}", su.page_size);
        },
        None => {},
    }
}
