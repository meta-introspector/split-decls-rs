macro_rules! cfg_rt {
    () => {
        macro_rules ! cfg_rt { ($ ($ item : item) *) => { $ (# [cfg (feature = "rt")] # [cfg_attr (docsrs , doc (cfg (feature = "rt")))] $ item) * } }
    };
}

cfg_rt!()