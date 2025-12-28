macro_rules! maybe_polyfill {
    () => {
        # [cfg_attr (feature = "std" , path = "maybe_polyfill/std/mod.rs")] # [cfg_attr (not (feature = "std") , path = "maybe_polyfill/no_std/mod.rs")] pub (crate) mod maybe_polyfill ;
    };
}

maybe_polyfill!();