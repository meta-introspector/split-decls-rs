macro_rules! cfg_not_unstable_metrics {
    () => {
        macro_rules ! cfg_not_unstable_metrics { ($ ($ item : item) *) => { $ (# [cfg (not (tokio_unstable))] $ item) * } }
    };
}

cfg_not_unstable_metrics!()