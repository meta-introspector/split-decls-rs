macro_rules! cfg_codec {
    () => {
        macro_rules ! cfg_codec { ($ ($ item : item) *) => { $ (# [cfg (feature = "codec")] # [cfg_attr (docsrs , doc (cfg (feature = "codec")))] $ item) * } }
    };
}

cfg_codec!();