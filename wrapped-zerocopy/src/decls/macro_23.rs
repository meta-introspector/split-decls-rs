macro_rules! macro_23 {
    () => {
        # [rustfmt :: skip] # [cfg (feature = "float-nightly")] impl_known_layout ! (# [cfg_attr (doc_cfg , doc (cfg (feature = "float-nightly")))] f16 , # [cfg_attr (doc_cfg , doc (cfg (feature = "float-nightly")))] f128) ;
    };
}

macro_23!()