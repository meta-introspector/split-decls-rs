macro_rules! typeid {
    () => {
        # [cfg (feature = "fs")] pub (crate) mod typeid ;
    };
}

typeid!();