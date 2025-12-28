macro_rules! cfg_not_rt_multi_thread {
    () => {
        macro_rules ! cfg_not_rt_multi_thread { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "rt-multi-thread"))] $ item) * } }
    };
}

cfg_not_rt_multi_thread!();