/// Cast a POD to a byte slice of itself
///
/// # Safety
/// You *must* know the alignment of T precisely.
/// Any padding bytes will mess up your plans.
/// Any T must be a POD = Plain Old Datatype, that is, a structure in the C sense.
pub unsafe fn cast_pod<T>(src: &T) -> &[u8] {
    core::slice::from_raw_parts(src as *const _ as *const u8, core::mem::size_of::<T>())
}

/// Cast a slice to a byte slice of itself
///
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

/// volatile write
///
/// Write a byte to `address`
/// # Safety
/// Make sure the address you write to is correct.
pub unsafe fn vwrite(address: u64, val: u8) {
    (address as *mut u8).write_volatile(val);
}

/// volatile read
///
/// Read a byte from `address`.
/// # Safety
/// Make sure the address you read from is correct.
pub unsafe fn vread(address: u64) -> u8 {
    (address as *mut u8).read_volatile()
}

/// volatile memory write
///
/// Write `count` bytes from `src` into `address`.
/// # Safety
/// Make sure the address you write to and the count are correct.
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

/// volatile memory write, iterator
///
/// Use this if you can't use `vmemwrite`.
/// Write `count` bytes from an iterator `src` into `address`.
/// # Safety
/// Make sure that the address you write to and the count are correct.
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

/// volatile memory read
///
/// Read `count` bytes from `address` into a buffer `dst`.
/// # Safety
/// Make sure that the address you read from and the count are correct.
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
