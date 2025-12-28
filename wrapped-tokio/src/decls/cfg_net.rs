macro_rules! cfg_net {
    () => {
        macro_rules ! cfg_net { ($ ($ item : item) *) => { $ (# [cfg (feature = "net")] # [cfg_attr (docsrs , doc (cfg (feature = "net")))] $ item) * } }
    };
}

cfg_net!()