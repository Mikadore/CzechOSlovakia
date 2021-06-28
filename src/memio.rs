/// Cast a POD to a byte slice of itself
/// # Safety
/// You *must* know the alignment of T precisely.
/// Any padding bytes will mess up your plans.
/// Any T must be a POD = Plain Old Datatype, that is, a structure in the C sense.
pub unsafe fn cast_pod<T>(src: &T) -> &[u8] {
    core::slice::from_raw_parts(src as *const _ as *const u8, core::mem::size_of::<T>())
}

/// Cast a slice to a byte slice of itself
/// # Safety
/// The same requirements for T apply:
/// You *must* know the alignment of T precisely.
/// Any padding bytes will mess up your plans.
/// Any T must be a POD = Plain Old Datatype, that is, a structure in the C sense.
pub unsafe fn cast_slice<T>(src: &[T]) -> &[u8] {
    core::slice::from_raw_parts(
        src.as_ptr() as *const u8,
        src.len() * core::mem::size_of::<T>(),
    )
}

/// Read a byte from an MMIO `port` 
/// # Safety
/// Validate that the port you write to is correct.
pub unsafe fn mmio_outb(port: u16, val: u8) {
    asm!("
        out dx, al
    ", in("dx") port, in("al") val);
}

/// Read a byte from an MMIO `port`.
/// # Safety
/// Validate that the port you read from is correct.
pub unsafe fn mmio_inb(port: u16) -> u8 {
    let mut out: u8;
    asm!("
        in al, dx
    ", in("dx") port, out("al") out);
    out
}

/// Write the bytes of `val` to `address
/// # Safety  
/// Validate that the address is correct.
/// Make *really* sure the objects size is the corret amount of bytes to write,
/// any padding bytes or other layout issues will mess this up.
#[unroll::unroll_for_loops]
pub unsafe fn vwrite<T>(address: u64, val: &T) {
    let ptr = val as *const _ as *const u8;
    for i in 0..core::mem::size_of::<T>() {
        (address as *mut u8).add(i).write_volatile(ptr.add(i).read());
    }
}

/// Write `count` bytes from `src` into `address`.
/// # Safety
/// Validate that the address you write to and the count are correct.
pub unsafe fn vmemwrite(address: u64, src: &[u8], count: usize) {
    if src.len() < count {
        panic!(
            "vmemwrite(0x{:x}, 0x{:x}, {}): src not big enough",
            address,
            src.as_ptr() as u64,
            count
        );
    } else {
        for (i, val) in src.iter().enumerate() {
            (address as *mut u8).add(i).write_volatile(*val);
        }
    }
}

/// Use this if you can't use `vmemwrite`.
/// Write `count` bytes from an iterator `src` into `address`.
/// # Safety
/// Validate that that the address you write to and the count are correct.
pub unsafe fn vmemwrite_iter(address: u64, mut src: impl Iterator<Item = u8>, count: usize) {
    let mut i = 0;
    while i < count {
        match src.next() {
            Some(b) => {
                (address as *mut u8).add(i).write_volatile(b);
            }
            None => {
                panic!(
                    "vmemwrite(0x{:x}, {{iterator}}, {}): src not big enough",
                    address, count
                );
            }
        }
        i += 1;
    }
}

/// Read `count` bytes from `address` into a buffer `dst`.
/// # Safety
/// Validate that that the address you read from and the count are correct.
pub unsafe fn vmemread(address: u64, dst: &mut [u8], count: usize) {
    if dst.len() < count {
        panic!(
            "vmemread(0x{:x}, 0x{:x}, {}): src not big enough",
            address,
            dst.as_ptr() as u64,
            count
        );
    } else {
        for (i, v) in dst.iter_mut().enumerate() {
            *v = (address as *mut u8).add(i).read_volatile();
        }
    }
}

/// Sets `count` bytes at `address` to `val`
/// # Safety
/// Validate that that the address you write to is correct.
pub unsafe fn vmemset(address: u64, val: u8, count: usize) {
    for i in 0..count {
        (address as *mut u8).add(i).write_volatile(val)
    }
} 
