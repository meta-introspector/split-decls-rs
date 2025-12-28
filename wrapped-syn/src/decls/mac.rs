macro_rules! mac {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod mac ;
    };
}

mac!();