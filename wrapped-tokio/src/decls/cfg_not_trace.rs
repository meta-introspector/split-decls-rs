macro_rules! cfg_not_trace {
    () => {
        macro_rules ! cfg_not_trace { ($ ($ item : item) *) => { $ (# [cfg (any (not (tokio_unstable) , not (feature = "tracing")))] $ item) * } }
    };
}

cfg_not_trace!()