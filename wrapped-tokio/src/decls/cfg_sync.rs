macro_rules! cfg_sync {
    () => {
        macro_rules ! cfg_sync { ($ ($ item : item) *) => { $ (# [cfg (feature = "sync")] # [cfg_attr (docsrs , doc (cfg (feature = "sync")))] $ item) * } }
    };
}

cfg_sync!()