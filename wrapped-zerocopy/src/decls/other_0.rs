macro_rules! other_0 {
    () => {
        # [cfg (any (feature = "derive" , test))] extern crate self as zerocopy ;
    };
}

other_0!();