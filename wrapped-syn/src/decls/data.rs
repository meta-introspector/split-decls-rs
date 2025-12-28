macro_rules! data {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod data ;
    };
}

data!();