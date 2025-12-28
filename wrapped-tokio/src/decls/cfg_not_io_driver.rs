macro_rules! cfg_not_io_driver {
    () => {
        macro_rules ! cfg_not_io_driver { ($ ($ item : item) *) => { $ (# [cfg (not (any (feature = "net" , all (unix , feature = "process") , all (unix , feature = "signal") , all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux"))))] $ item) * } }
    };
}

cfg_not_io_driver!()