use nix::sys::socket::{socket, connect};
use nix::sys::socket::{AddressFamily, SockType, SockFlag, SockAddr, InetAddr};
use nix::unistd::{read, close};


use std::env::args;
use std::ffi::CStr;
use std::os::unix::prelude::RawFd;
use std::process::exit;
use std::net::SocketAddr;
fn main() {
    // check the cli args
    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        eprintln!("Usage: {} <ip> <port>", av[0]);
        exit(1);
    }

    let sock: RawFd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    ).expect("socket() error");

    let serv_addr_std: SocketAddr = format!("{}:{}", av[1], av[2]).parse().expect("not a valid address");
    let inet_serv_addr: InetAddr = InetAddr::from_std(&serv_addr_std);

    let serv_addr: SockAddr = SockAddr::Inet(inet_serv_addr);

    connect(sock, &serv_addr).expect("connect() error");

    let mut buf: [u8; 30] = [0; 30];
    let mut index: usize = 0;
    while let Ok(n) = read(sock, &mut buf[index..index+1]) {
        // reaching EOF
        if n == 0 {
            break;
        }
        index += n;
    }

    let msg: &CStr = CStr::from_bytes_with_nul(&buf[0..=index]).expect("not a valid c string");
    println!("message from server: {:?}", msg);
    println!("function read call count: {}",  index);
    let _ = close(sock);
}
