macro_rules! cfg_net_or_process {
    () => {
        macro_rules ! cfg_net_or_process { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , feature = "process"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "net" , feature = "process"))))] $ item) * } }
    };
}

cfg_net_or_process!();