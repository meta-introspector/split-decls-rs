macro_rules! cfg_signal {
    () => {
        macro_rules ! cfg_signal { ($ ($ item : item) *) => { $ (# [cfg (feature = "signal")] # [cfg_attr (docsrs , doc (cfg (feature = "signal")))] # [cfg (not (loom))] # [cfg (not (target_os = "wasi"))] $ item) * } }
    };
}

cfg_signal!()