macro_rules! cfg_signal_internal {
    () => {
        macro_rules ! cfg_signal_internal { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "signal" , all (unix , feature = "process")))] # [cfg (not (loom))] $ item) * } }
    };
}

cfg_signal_internal!()