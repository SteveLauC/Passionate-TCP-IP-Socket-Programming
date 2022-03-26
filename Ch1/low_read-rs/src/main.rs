use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::read;

use std::os::unix::prelude::RawFd;

fn main() {
    let fd: RawFd =
        open("data.txt", OFlag::O_RDONLY, Mode::empty()).expect("Can not open data.txt");
    let mut buf: [u8; 10] = [0_u8; 10];
    read(fd, &mut buf).expect("Cannot read from data.txt");


    // weird way to print this string.
    for byte in buf {
        if byte != 0 {
            print!("{}", char::from(byte));
        }
    }

    /*
        if I use

        ```rsut
        CStr::from_bytes_with_nul(&buf).expect("Not a valid c string");
        ```
        to create the c style string, the program will panic cuz there are more
        than one NUL in the tail of the array:(
    */
}