macro_rules! other_0 {
    () => {
        # [cfg (any (feature = "rust-allocator" , feature = "c-allocator"))] extern crate alloc ;
    };
}

other_0!();