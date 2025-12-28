macro_rules! cfg_not_time {
    () => {
        macro_rules ! cfg_not_time { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "time"))] $ item) * } }
    };
}

cfg_not_time!();