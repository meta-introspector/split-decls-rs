macro_rules! cfg_not_coop {
    () => {
        macro_rules ! cfg_not_coop { ($ ($ item : item) *) => { $ (# [cfg (not (any (feature = "fs" , feature = "io-std" , feature = "net" , feature = "process" , feature = "rt" , feature = "signal" , feature = "sync" , feature = "time" ,)))] $ item) * } }
    };
}

cfg_not_coop!()