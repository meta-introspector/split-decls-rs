macro_rules! rand {
    () => {
        # [cfg (any (feature = "rt" , feature = "macros"))] pub (crate) mod rand ;
    };
}

rand!();