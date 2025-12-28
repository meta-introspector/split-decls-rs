macro_rules! cfg_not_rt_and_metrics_and_net {
    () => {
        macro_rules ! cfg_not_rt_and_metrics_and_net { ($ ($ item : item) *) => { $ (# [cfg (not (all (feature = "net" , feature = "rt" , tokio_unstable)))] $ item) * } }
    };
}

cfg_not_rt_and_metrics_and_net!();