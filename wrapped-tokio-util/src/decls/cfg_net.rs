macro_rules! cfg_net {
    () => {
        macro_rules ! cfg_net { ($ ($ item : item) *) => { $ (# [cfg (all (feature = "net" , feature = "codec"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "net" , feature = "codec"))))] $ item) * } }
    };
}

cfg_net!();