macro_rules! cfg_not_io_util {
    () => {
        macro_rules ! cfg_not_io_util { ($ ($ item : item) *) => { $ (# [cfg (not (feature = "io-util"))] $ item) * } }
    };
}

cfg_not_io_util!()