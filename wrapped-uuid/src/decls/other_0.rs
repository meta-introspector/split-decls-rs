macro_rules! other_0 {
    () => {
        # [cfg (any (feature = "std" , test))] # [macro_use] extern crate std ;
    };
}

other_0!();