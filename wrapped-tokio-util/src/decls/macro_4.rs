macro_rules! macro_4 {
    () => {
        cfg_io ! { macro_rules ! cfg_io_util { ($ ($ item : item) *) => { $ (# [cfg (feature = "io-util")] # [cfg_attr (docsrs , doc (cfg (feature = "io-util")))] $ item) * } } }
    };
}

macro_4!();