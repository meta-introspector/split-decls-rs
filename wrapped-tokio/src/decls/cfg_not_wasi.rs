macro_rules! cfg_not_wasi {
    () => {
        macro_rules ! cfg_not_wasi { ($ ($ item : item) *) => { $ (# [cfg (not (target_os = "wasi"))] $ item) * } }
    };
}

cfg_not_wasi!();