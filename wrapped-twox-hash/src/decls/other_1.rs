macro_rules! other_1 {
    () => {
        # [cfg (any (feature = "std" , doc , test))] extern crate std ;
    };
}

other_1!()