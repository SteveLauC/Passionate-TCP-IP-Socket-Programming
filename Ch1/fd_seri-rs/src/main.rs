use nix::fcntl::{open, OFlag};
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
use nix::sys::stat::Mode;
use nix::unistd::close;
use std::os::unix::io::RawFd;

fn main() {
    let fd1: RawFd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("can not create socket file");
    let fd2: RawFd = open(
        "test.txt",
        OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC,
        Mode::from_bits(0o644_u32).expect("can not create Mode type from this Oct literal"),
    )
    .expect("can not open file");
    let fd3: RawFd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("can not create socket file");

    println!("file descriptor 1: {}", fd1);
    println!("file descriptor 2: {}", fd2);
    println!("file descriptor 3: {}", fd3);

    let _ = close(fd1);
    let _ = close(fd2);
    let _ = close(fd3);
}
