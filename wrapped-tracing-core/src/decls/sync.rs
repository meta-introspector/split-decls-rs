macro_rules! sync {
    () => {
        # [cfg (not (feature = "std"))] mod sync ;
    };
}

sync!();