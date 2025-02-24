use libc::{c_char, c_short, AF_INET, SOCK_DGRAM};
use std::ffi::CString;
use std::io;

#[repr(C)]
struct Ifreq {
    ifr_name: [c_char; libc::IFNAMSIZ],
    ifr_flags: c_short,
}

// if let Err(e) = set_interface_up("eth0") { ... }
pub fn set_interface_up(interface_name: &str) -> io::Result<()> {
    let sock = unsafe { libc::socket(AF_INET, SOCK_DGRAM, 0) };
    if sock < 0 {
        return Err(io::Error::last_os_error());
    }
    let mut ifr = Ifreq {
        ifr_name: [0; libc::IFNAMSIZ],
        ifr_flags: 0,
    };
    let cstr = CString::new(interface_name).unwrap();
    for (i, &c) in cstr.as_bytes().iter().enumerate() {
        ifr.ifr_name[i] = c as c_char;
    }
    unsafe {
        if libc::ioctl(sock, 0x8913, &mut ifr) < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    ifr.ifr_flags |= 1;
    unsafe {
        if libc::ioctl(sock, 0x8914, &ifr) < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    println!("Interface {} is set up.", interface_name);
    Ok(())
}
