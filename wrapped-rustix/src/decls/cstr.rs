macro_rules! cstr {
    () => {
        # [cfg (not (windows))] # [macro_use] pub (crate) mod cstr ;
    };
}

cstr!();