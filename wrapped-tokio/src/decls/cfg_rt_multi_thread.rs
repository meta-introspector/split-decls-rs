macro_rules! cfg_rt_multi_thread {
    () => {
        macro_rules ! cfg_rt_multi_thread { ($ ($ item : item) *) => { $ (# [cfg (feature = "rt-multi-thread")] # [cfg_attr (docsrs , doc (cfg (feature = "rt-multi-thread")))] $ item) * } }
    };
}

cfg_rt_multi_thread!()