macro_rules! other_1 {
    () => {
        # [cfg (not (feature = "rustc-dep-of-std"))] extern crate alloc ;
    };
}

other_1!()