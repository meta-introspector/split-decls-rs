macro_rules! other_0 {
    () => {
        # [cfg (not (feature = "std"))] extern crate alloc ;
    };
}

other_0!();