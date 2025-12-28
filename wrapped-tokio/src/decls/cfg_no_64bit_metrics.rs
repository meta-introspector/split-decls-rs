macro_rules! cfg_no_64bit_metrics {
    () => {
        macro_rules ! cfg_no_64bit_metrics { ($ ($ item : item) *) => { $ (# [cfg (not (target_has_atomic = "64"))] $ item) * } }
    };
}

cfg_no_64bit_metrics!();