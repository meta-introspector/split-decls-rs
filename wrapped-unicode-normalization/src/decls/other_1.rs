macro_rules! other_1 {
    () => {
        # [cfg (feature = "std")] extern crate core ;
    };
}

other_1!()