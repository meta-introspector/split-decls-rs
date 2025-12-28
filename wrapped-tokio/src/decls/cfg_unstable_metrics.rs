macro_rules! cfg_unstable_metrics {
    () => {
        macro_rules ! cfg_unstable_metrics { ($ ($ item : item) *) => { $ (# [cfg (tokio_unstable)] # [cfg_attr (docsrs , doc (cfg (tokio_unstable)))] $ item) * } }
    };
}

cfg_unstable_metrics!()