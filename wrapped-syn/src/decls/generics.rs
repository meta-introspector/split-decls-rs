macro_rules! generics {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod generics ;
    };
}

generics!();