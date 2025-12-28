macro_rules! cfg_net_or_uring {
    () => {
        macro_rules ! cfg_net_or_uring { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux" ,)))] # [cfg_attr (docsrs , doc (cfg (any (feature = "net" , all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux" ,)))))] $ item) * } }
    };
}

cfg_net_or_uring!();