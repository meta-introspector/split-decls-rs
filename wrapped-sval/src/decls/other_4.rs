macro_rules! other_4 {
    () => {
        # [cfg (all (not (feature = "alloc") , not (feature = "std")))] extern crate core as std ;
    };
}

other_4!();