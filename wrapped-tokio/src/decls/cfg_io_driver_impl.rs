macro_rules! cfg_io_driver_impl {
    () => {
        macro_rules ! cfg_io_driver_impl { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , all (unix , feature = "process") , all (unix , feature = "signal") , all (tokio_unstable , feature = "io-uring" , feature = "rt" , feature = "fs" , target_os = "linux")))] $ item) * } }
    };
}

cfg_io_driver_impl!()