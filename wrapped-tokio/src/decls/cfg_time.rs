macro_rules! cfg_time {
    () => {
        macro_rules ! cfg_time { ($ ($ item : item) *) => { $ (# [cfg (feature = "time")] # [cfg_attr (docsrs , doc (cfg (feature = "time")))] $ item) * } }
    };
}

cfg_time!()