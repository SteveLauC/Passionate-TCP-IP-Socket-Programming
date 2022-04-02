mod hton;
use hton::htonx;

use std::env::args;
use std::os::unix::io::RawFd;

use nix::sys::socket::{accept, bind, listen, socket};
use nix::sys::socket::{sockaddr_in, AddressFamily, InetAddr, SockAddr, SockFlag, SockType};

use nix::unistd::close;
use nix::unistd::write;

use libc::in_addr;

fn main() {
    // cmd args handling to get the port number
    let av: Vec<String> = args().collect();
    if av.len() != 2 {
        eprintln!("Usage: {} <port>", av[0]);
    }

    let serv_sock: RawFd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("socket() error");

    let serv_addr: SockAddr = SockAddr::Inet(InetAddr::V4(sockaddr_in {
        sin_family: AddressFamily::Inet as u16,
        sin_addr: in_addr { s_addr: htonx(0) },
        sin_port: htonx(av[1].parse().expect("not a valid port number")),
        sin_zero: [0; 8],
    }));

    bind(serv_sock, &serv_addr).expect("can not bind server socket to the corresponding address");
    listen(serv_sock, 5).expect("can not activate the server side");
    let clnt_sock: RawFd = accept(serv_sock).expect("can not accept request");

    let msg: &str = "hello world";
    write(clnt_sock, msg.as_bytes()).expect("can not write to the client side");

    let _ = close(serv_sock);
    let _ = close(clnt_sock);
}
