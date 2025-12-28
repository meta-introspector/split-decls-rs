macro_rules! other_1 {
    () => {
        # [cfg (all (not (feature = "std") , not (test)))] # [macro_use] extern crate core as std ;
    };
}

other_1!();