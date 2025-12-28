macro_rules! other_2 {
    () => {
        # [cfg (feature = "std")] extern crate std as core ;
    };
}

other_2!();