macro_rules! cfg_unstable_windows {
    () => {
        # [doc = " Enables unstable Windows-specific code."] # [doc = " Use this macro instead of `cfg(windows)` to generate docs properly."] macro_rules ! cfg_unstable_windows { ($ ($ item : item) *) => { $ (# [cfg (all (any (all (doc , docsrs) , windows) , tokio_unstable))] # [cfg_attr (docsrs , doc (cfg (all (windows , tokio_unstable))))] $ item) * } }
    };
}

cfg_unstable_windows!();