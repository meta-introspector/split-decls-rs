macro_rules! cfg_not_loom {
    () => {
        macro_rules ! cfg_not_loom { ($ ($ item : item) *) => { $ (# [cfg (not (loom))] $ item) * } }
    };
}

cfg_not_loom!()