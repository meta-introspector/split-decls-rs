macro_rules! cfg_io_readiness {
    () => {
        macro_rules ! cfg_io_readiness { ($ ($ item : item) *) => { $ (# [cfg (feature = "net")] $ item) * } }
    };
}

cfg_io_readiness!()