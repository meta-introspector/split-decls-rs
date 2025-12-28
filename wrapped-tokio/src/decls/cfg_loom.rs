macro_rules! cfg_loom {
    () => {
        macro_rules ! cfg_loom { ($ ($ item : item) *) => { $ (# [cfg (loom)] $ item) * } }
    };
}

cfg_loom!()