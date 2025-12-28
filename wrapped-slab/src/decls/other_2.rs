macro_rules! other_2 {
    () => {
        # [cfg (feature = "std")] extern crate std as alloc ;
    };
}

other_2!()