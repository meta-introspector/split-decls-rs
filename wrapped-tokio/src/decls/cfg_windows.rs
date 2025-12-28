macro_rules! cfg_windows {
    () => {
        # [doc = " Enables Windows-specific code."] # [doc = " Use this macro instead of `cfg(windows)` to generate docs properly."] macro_rules ! cfg_windows { ($ ($ item : item) *) => { $ (# [cfg (any (all (doc , docsrs) , windows))] # [cfg_attr (docsrs , doc (cfg (windows)))] $ item) * } }
    };
}

cfg_windows!()