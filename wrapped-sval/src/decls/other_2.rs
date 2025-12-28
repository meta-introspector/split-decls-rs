macro_rules! other_2 {
    () => {
        # [cfg (all (feature = "alloc" , not (feature = "std")))] extern crate core ;
    };
}

other_2!()