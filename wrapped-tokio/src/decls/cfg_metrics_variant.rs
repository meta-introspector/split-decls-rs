macro_rules! cfg_metrics_variant {
    () => {
        # [doc = " Use this macro to provide two different implementations of the same API — one for stable"] # [doc = " builds and one for unstable builds."] macro_rules ! cfg_metrics_variant { (stable : { $ ($ stable_code : tt) * } , unstable : { $ ($ unstable_code : tt) * }) => { cfg_not_unstable_metrics ! { $ ($ stable_code) * } cfg_unstable_metrics ! { $ ($ unstable_code) * } } }
    };
}

cfg_metrics_variant!();