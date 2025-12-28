macro_rules! cfg_64bit_metrics {
    () => {
        # [doc = " Some metrics require 64-bit atomics."] macro_rules ! cfg_64bit_metrics { ($ ($ item : item) *) => { $ (# [cfg (target_has_atomic = "64")] # [cfg_attr (docsrs , doc (cfg (target_has_atomic = "64")))] $ item) * } }
    };
}

cfg_64bit_metrics!()