macro_rules! cfg_not_signal_internal {
    () => {
        macro_rules ! cfg_not_signal_internal { ($ ($ item : item) *) => { $ (# [cfg (any (loom , not (unix) , not (any (feature = "signal" , all (unix , feature = "process")))))] $ item) * } }
    };
}

cfg_not_signal_internal!()