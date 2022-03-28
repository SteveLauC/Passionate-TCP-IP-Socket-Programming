use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::read;

use std::ffi::CStr;
use std::os::unix::prelude::RawFd;

fn main() {
    let fd: RawFd =
        open("data.txt", OFlag::O_RDONLY, Mode::empty()).expect("Can not open data.txt");

    let mut buf: [u8; 10] = [0_u8; 10];

    let n_bytes: usize = read(fd, &mut buf).expect("Cannot read from data.txt");
    let contents: &CStr =
        CStr::from_bytes_with_nul(&buf[0..=n_bytes]).expect("Not a valid c string");
    println!("file data: {:?}", contents);
}
