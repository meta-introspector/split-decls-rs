macro_rules! ty {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod ty ;
    };
}

ty!();