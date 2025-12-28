macro_rules! pat {
    () => {
        # [cfg (feature = "full")] mod pat ;
    };
}

pat!();