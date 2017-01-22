extern crate libc;
use libc::{c_short, pid_t, c_char, int32_t, c_int, size_t, c_void};

use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::mem;

pub const UT_LINESIZE: usize = 32;
pub const UT_NAMESIZE: usize = 32;
pub const UT_HOSTSIZE: usize = 256;
static UTMP_FILE_PATH: &'static str = "/var/run/utmp";

#[repr(C)]
pub struct exit_status {
    pub e_termination: c_short,
    pub e_exit: c_short,
}

#[repr(C)]
pub struct ut_tv {
    pub tv_sec: int32_t,
    pub tv_usec: int32_t,
}

#[repr(C)]
pub struct utmp {
    pub ut_type: c_short,
    pub ut_pid: pid_t,
    pub ut_line: [c_char; UT_LINESIZE],
    pub ut_id: [c_char; 4],
    pub ut_user: [c_char; UT_NAMESIZE],
    pub ut_host: [c_char; UT_HOSTSIZE],
    pub ut_exit: exit_status,
    pub ut_session: int32_t,
    pub ut_tv: ut_tv,
    pub ut_addr_v6: [int32_t; 4],
    pub __glibc_reserved: [c_char; 20],
}

impl Default for exit_status {
    fn default() -> exit_status {
        exit_status {
            e_termination: 0,
            e_exit: 0,
        }
    }
}

impl Default for ut_tv {
    fn default() -> ut_tv {
        ut_tv {
            tv_sec: 0,
            tv_usec: 0,
        }
    }
}

impl Default for utmp {
    fn default() -> utmp {
        utmp {
            ut_type: 0,
            ut_pid: 0,
            ut_line: [0; UT_LINESIZE],
            ut_id: [0; 4],
            ut_user: [0; UT_NAMESIZE],
            ut_host: [0; UT_HOSTSIZE],
            ut_exit: Default::default(),
            ut_session: 0,
            ut_tv: Default::default(),
            ut_addr_v6: [0; 4],
            __glibc_reserved: [0; 20],
        }
    }
}

extern "C" {
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> usize;
}

fn show_info(utmp_struct: &utmp) {
    print!("{} ",
           String::from_utf8((utmp_struct.ut_user.iter().map(|&x| x as u8).collect())).unwrap());
    print!("{} ",
           String::from_utf8((utmp_struct.ut_line.iter().map(|&x| x as u8).collect())).unwrap());
    print!("{} ", utmp_struct.ut_tv.tv_sec);
    print!("({}) ",
           String::from_utf8((utmp_struct.ut_host.iter().map(|&x| x as u8).collect())).unwrap());
    println!("");
}


fn main() {
    let utmp_file = File::open(UTMP_FILE_PATH).unwrap();
    let mut utmp_struct: utmp = Default::default();
    let buffer: *mut c_void = &mut utmp_struct as *mut _ as *mut c_void;
    unsafe {
        while read(utmp_file.as_raw_fd(), buffer, mem::size_of::<utmp>()) != 0 {
            show_info(&utmp_struct);
        }
    }
}
