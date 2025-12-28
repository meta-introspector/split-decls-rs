macro_rules! dup {
    () => {
        # [cfg (not (windows))] mod dup ;
    };
}

dup!();