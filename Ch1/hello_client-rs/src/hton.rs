use std::mem;

// This funciton is not platform-agnostic, and only works on little
// endian machines
pub fn htonx<T>(mut hostx: T) -> T {
    let mut tmp: u8;
    let size: usize = mem::size_of_val(&hostx);

    // two pointers, ready for swap
    let mut head_p: *mut u8 = &mut hostx as *mut T as *mut u8;
    let mut tail_p: *mut u8 = unsafe { head_p.add(size - 1) };

    for i in 0..size / 2 {
        unsafe {
            head_p = head_p.add(i);
            tail_p = tail_p.sub(i);

            tmp = *head_p;
            *head_p = *tail_p;
            *tail_p = tmp;
        }
    }

    hostx
}
