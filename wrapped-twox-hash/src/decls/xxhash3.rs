macro_rules! xxhash3 {
    () => {
        # [cfg (any (feature = "xxhash3_64" , feature = "xxhash3_128"))] mod xxhash3 ;
    };
}

xxhash3!()