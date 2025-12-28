macro_rules! cfg_io_uring {
    () => {
        macro_rules ! cfg_io_uring { ($ ($ item : item) *) => { $ (# [cfg (all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux" ,))] $ item) * } ; }
    };
}

cfg_io_uring!()