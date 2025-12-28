macro_rules! cfg_signal {
    () => {
        macro_rules ! cfg_signal { ($ ($ item : item) *) => { $ (# [cfg (feature = "signal")] # [cfg_attr (docsrs , doc (cfg (feature = "signal")))] $ item) * } }
    };
}

cfg_signal!();