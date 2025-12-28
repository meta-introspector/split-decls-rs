macro_rules! cfg_trace {
    () => {
        macro_rules ! cfg_trace { ($ ($ item : item) *) => { $ (# [cfg (all (tokio_unstable , feature = "tracing"))] # [cfg_attr (docsrs , doc (cfg (all (tokio_unstable , feature = "tracing"))))] $ item) * } ; }
    };
}

cfg_trace!();