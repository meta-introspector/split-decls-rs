macro_rules! map {
    () => {
        # [cfg (any (feature = "std" , feature = "mini_moka" , feature = "hashbrown"))] mod map ;
    };
}

map!();