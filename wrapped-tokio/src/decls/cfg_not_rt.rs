macro_rules! cfg_not_rt {
    () => {
        macro_rules ! cfg_not_rt { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "rt"))] $ item) * } }
    };
}

cfg_not_rt!()