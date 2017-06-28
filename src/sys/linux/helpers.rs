use libc;

pub fn sysinfo() -> libc::sysinfo {
    let mut si = libc::sysinfo {
        uptime: 0,
        loads: [0, 0, 0],
        totalram: 0,
        freeram: 0,
        sharedram: 0,
        bufferram: 0,
        totalswap: 0,
        freeswap: 0,
        procs: 0,
        pad: 0,
        totalhigh: 0,
        freehigh: 0,
        mem_unit: 0,
        _f: [],
    };
    unsafe {
        libc::sysinfo(&mut si);
    }
    si
}
