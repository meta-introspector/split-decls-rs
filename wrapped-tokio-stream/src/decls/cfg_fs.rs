macro_rules! cfg_fs {
    () => {
        macro_rules ! cfg_fs { ($ ($ item : item) *) => { $ (# [cfg (feature = "fs")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] $ item) * } }
    };
}

cfg_fs!();