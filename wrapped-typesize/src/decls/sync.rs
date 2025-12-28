macro_rules! sync {
    () => {
        # [cfg (feature = "std")] mod sync ;
    };
}

sync!();