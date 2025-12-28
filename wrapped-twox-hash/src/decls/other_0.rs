macro_rules! other_0 {
    () => {
        # [cfg (all (feature = "alloc" , any (feature = "xxhash3_64" , feature = "xxhash3_128")))] extern crate alloc ;
    };
}

other_0!()