macro_rules! cfg_not_sync {
    () => {
        macro_rules ! cfg_not_sync { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "sync"))] $ item) * } }
    };
}

cfg_not_sync!()