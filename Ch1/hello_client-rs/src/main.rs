mod hton;
use hton::htonx;
use nix::libc::in_addr;

use nix::sys::socket::{connect, sockaddr_in, InetAddr, SockAddr};
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
use nix::unistd::{close, read};

use std::env::args;
use std::ffi::CStr;
use std::os::unix::io::RawFd;
use std::process;

fn main() {
    let mut buf: [u8; 30] = [0; 30];

    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        eprintln!("usage: {} <IP> <PORT>", av[0]);
        process::exit(1);
    }

    // get client side file descriptor
    let sock: RawFd = socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        None,
    )
    .expect("cannot create client socket file");

    // server address
    let serv_addr: SockAddr = SockAddr::new_inet(InetAddr::V4(sockaddr_in {
        sin_family: AddressFamily::Inet as u16,
        sin_port: htonx((av[2].parse::<u16>()).unwrap()),
        sin_addr: in_addr {
            s_addr: htonx(av[1].parse::<u32>().unwrap()),
        },
        sin_zero: [0; 8],
    }));

    // initiate a connection
    if let Err(msg) = connect(sock, &serv_addr) {
        eprintln!("{}", msg);
        process::exit(1);
    }

    let n_bytes = read(sock, &mut buf).expect("can not read from sock");
    let msg: &CStr = CStr::from_bytes_with_nul(&buf[0..n_bytes]).expect("Not a valid c string");

    println!("message from server: {:?}", msg);
    let _ = close(sock);
}
