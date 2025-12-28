macro_rules! deps {
    () => {
        SixtyFourBitPointer!();
    };
}

macro_rules! other_72 {
    () => {
        deps!();
        # [doc = " Data associated with an [`epoll::Event`]. This can either be a 64-bit"] # [doc = " integer value or a pointer which preserves pointer provenance."] # [repr (C)] # [derive (Copy , Clone)] pub union EventData { # [doc = " A 64-bit integer value."] as_u64 : u64 , # [doc = " A `*mut c_void` which preserves pointer provenance, extended to be"] # [doc = " 64-bit so that if we read the value as a `u64` union field, we don't"] # [doc = " get uninitialized memory."] sixty_four_bit_pointer : SixtyFourBitPointer , }
    };
}

other_72!()