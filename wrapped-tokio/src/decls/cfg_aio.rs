macro_rules! cfg_aio {
    () => {
        macro_rules ! cfg_aio { ($ ($ item : item) *) => { $ (# [cfg (all (any (docsrs , target_os = "freebsd") , feature = "net"))] # [cfg_attr (docsrs , doc (cfg (all (target_os = "freebsd" , feature = "net"))))] $ item) * } }
    };
}

cfg_aio!()