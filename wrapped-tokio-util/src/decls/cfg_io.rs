macro_rules! cfg_io {
    () => {
        macro_rules ! cfg_io { ($ ($ item : item) *) => { $ (# [cfg (feature = "io")] # [cfg_attr (docsrs , doc (cfg (feature = "io")))] $ item) * } }
    };
}

cfg_io!()