mod htonx;
use htonx::htonx;

use std::env::args;
use std::os::unix::prelude::RawFd;
use std::process::exit;

use nix::sys::socket::{accept, bind, listen, socket};
use nix::sys::socket::{sockaddr_in, AddressFamily, InetAddr, SockAddr, SockFlag, SockType};
use nix::unistd::{close, write};

use libc::in_addr;

fn main() {
    // check the cli args
    let av: Vec<String> = args().collect();
    if av.len() != 2 {
        eprintln!("Usage: {} <port>", av[0]);
        exit(1);
    }

    // get the server file descriptor
    let serv_sock: RawFd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("socket() error");

    // initiate server address
    let serv_addr: SockAddr = SockAddr::Inet(InetAddr::V4(sockaddr_in {
        sin_family: AddressFamily::Inet as u16,
        sin_addr: in_addr { s_addr: 0 },
        sin_port: htonx(av[1].parse::<u16>().expect("not a valid port number")),
        sin_zero: [0; 8],
    }));

    // bind the address to server file descriptor
    bind(serv_sock, &serv_addr).expect("bind() error");

    // activate the server
    listen(serv_sock, 5).expect("listen() error");

    // accept the request
    let clnt_sock: RawFd = accept(serv_sock).expect("accept() error");

    let message: &str = "hello world";
    write(clnt_sock, message.as_bytes()).expect("write() error");
    let _ = close(serv_sock);
    let _ = close(clnt_sock);
}
