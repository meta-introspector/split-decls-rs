macro_rules! other_1 {
    () => {
        # [cfg (all (feature = "alloc" , not (feature = "rustc-dep-of-std")))] extern crate alloc ;
    };
}

other_1!();