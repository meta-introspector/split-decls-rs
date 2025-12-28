macro_rules! cfg_io_blocking {
    () => {
        macro_rules ! cfg_io_blocking { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "io-std" , feature = "fs" , all (windows , feature = "process") ,))] $ item) * } }
    };
}

cfg_io_blocking!()