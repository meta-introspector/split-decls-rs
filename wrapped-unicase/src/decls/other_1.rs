macro_rules! other_1 {
    () => {
        # [cfg (feature = "nightly")] extern crate test ;
    };
}

other_1!()