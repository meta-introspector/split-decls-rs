macro_rules! cfg_compat {
    () => {
        macro_rules ! cfg_compat { ($ ($ item : item) *) => { $ (# [cfg (feature = "compat")] # [cfg_attr (docsrs , doc (cfg (feature = "compat")))] $ item) * } }
    };
}

cfg_compat!()