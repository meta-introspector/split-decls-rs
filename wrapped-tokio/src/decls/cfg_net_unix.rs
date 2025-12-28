macro_rules! cfg_net_unix {
    () => {
        macro_rules ! cfg_net_unix { ($ ($ item : item) *) => { $ (# [cfg (all (unix , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (all (unix , feature = "net"))))] $ item) * } }
    };
}

cfg_net_unix!();