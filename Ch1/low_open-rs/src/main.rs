use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, write};
use std::os::unix::prelude::RawFd;

fn main() {
    let buf: &str = "let's go";
    let slice : &[u8] = buf.as_bytes();

    let fd: RawFd = open(
        "data.txt",
        OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
        Mode::from_bits(0o644_u32).unwrap(),
    )
    .expect("cannot open data.txt");

    println!("file descriptor: {}", fd);

    write(fd, slice).unwrap();


    let _ = close(fd);
}
