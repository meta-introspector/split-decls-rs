macro_rules! cfg_coop {
    () => {
        macro_rules ! cfg_coop { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "fs" , feature = "io-std" , feature = "net" , feature = "process" , feature = "rt" , feature = "signal" , feature = "sync" , feature = "time" ,))] $ item) * } }
    };
}

cfg_coop!();