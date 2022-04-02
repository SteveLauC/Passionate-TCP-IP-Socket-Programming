use nix::sys::socket::{connect, InetAddr, SockAddr};
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
use nix::unistd::{close, read};

use std::env::args;
use std::ffi::CStr;
use std::os::unix::io::RawFd;
use std::process;
use std::net::SocketAddr;


fn main() {
    let mut buf: [u8; 30] = [0; 30];

    let av: Vec<String> = args().collect();
    if av.len() != 3 {
        eprintln!("usage: {} <IP> <PORT>", av[0]);
        process::exit(1);
    }

    // init client file descriptor
    let sock: RawFd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("cannot create client socket file");

    
    // instantiate server address
    let addr_std: SocketAddr = format!("{}:{}", av[1], av[2]).parse().expect("not a valid ipv4 address");
    // server address
    let serv_addr: SockAddr = SockAddr::Inet(InetAddr::from_std(&addr_std));

    // establish a connection
    if let Err(msg) = connect(sock, &serv_addr) {
        eprintln!("{}", msg);
        process::exit(1);
    }

    let n_bytes = read(sock, &mut buf).expect("can not read from sock");
    let msg: &CStr = CStr::from_bytes_with_nul(&buf[0..=n_bytes]).expect("Not a valid c string");

    println!("message from server: {:?}", msg);
    let _ = close(sock);
}
