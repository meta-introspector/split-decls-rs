macro_rules! other_1 {
    () => {
        # [cfg (all (feature = "alloc" , not (feature = "std")))] extern crate alloc ;
    };
}

other_1!()