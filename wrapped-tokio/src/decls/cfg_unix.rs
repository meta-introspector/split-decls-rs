macro_rules! cfg_unix {
    () => {
        # [doc = " Enables Unix-specific code."] # [doc = " Use this macro instead of `cfg(unix)` to generate docs properly."] macro_rules ! cfg_unix { ($ ($ item : item) *) => { $ (# [cfg (any (all (doc , docsrs) , unix))] # [cfg_attr (docsrs , doc (cfg (unix)))] $ item) * } }
    };
}

cfg_unix!();