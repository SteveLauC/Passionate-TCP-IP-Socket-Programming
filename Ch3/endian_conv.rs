use std::mem;


// This funciton is not platform-agnostic, and only works on little
// endian machines
fn htonx<T>(mut hostx: T) -> T {
    let mut tmp: u8;
    let size: usize = mem::size_of_val(&hostx);

    // two pointers, ready for swap
    let mut head_p: *mut u8 = &mut hostx as *mut T as *mut u8;
    let mut tail_p: *mut u8 = unsafe{head_p.add(size-1)};

    for i in 0..size/2 {
        unsafe{
            head_p = head_p.add(i);
            tail_p = tail_p.sub(i);

            tmp = *head_p;
            *head_p = *tail_p;
            *tail_p = tmp;
        }        
    }

    hostx
}

fn main() {
    let host_port: u16 = 0x1234;
    let net_port: u16 = htonx(host_port);
    let host_ip: u32 = 0x12345678; 
    let net_ip: u32 = htonx(host_ip);

    println!("Host ordered port: {:x}", host_port);
    println!("Network ordered port: {:x}", net_port);
    println!("Host ordered address: {:x}", host_ip);
    println!("Network ordered address: {:x}", net_ip);
}