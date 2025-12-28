macro_rules! cfg_net_windows {
    () => {
        macro_rules ! cfg_net_windows { ($ ($ item : item) *) => { $ (# [cfg (all (any (all (doc , docsrs) , windows) , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (all (windows , feature = "net"))))] $ item) * } }
    };
}

cfg_net_windows!()