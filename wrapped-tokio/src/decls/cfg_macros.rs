macro_rules! cfg_macros {
    () => {
        macro_rules ! cfg_macros { ($ ($ item : item) *) => { $ (# [cfg (feature = "macros")] # [cfg_attr (docsrs , doc (cfg (feature = "macros")))] $ item) * } }
    };
}

cfg_macros!();