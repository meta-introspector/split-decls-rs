macro_rules! tt {
    () => {
        # [cfg (all (any (feature = "full" , feature = "derive") , feature = "extra-traits"))] mod tt ;
    };
}

tt!()