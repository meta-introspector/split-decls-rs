macro_rules! other_1 {
    () => {
        # [cfg (feature = "std")] extern crate std ;
    };
}

other_1!()