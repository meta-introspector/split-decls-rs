macro_rules! cfg_io_std {
    () => {
        macro_rules ! cfg_io_std { ($ ($ item : item) *) => { $ (# [cfg (feature = "io-std")] # [cfg_attr (docsrs , doc (cfg (feature = "io-std")))] $ item) * } }
    };
}

cfg_io_std!();