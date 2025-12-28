macro_rules! deps {
    () => {
        Layer!();
    };
}

macro_rules! FilterFn {
    () => {
        deps!();
        # [doc = " A filter implemented by a closure or function pointer that"] # [doc = " determines whether a given span or event is enabled, based on its"] # [doc = " [`Metadata`]."] # [doc = ""] # [doc = " This type can be used for both [per-layer filtering][plf] (using its"] # [doc = " [`Filter`] implementation) and [global filtering][global] (using its"] # [doc = " [`Layer`] implementation)."] # [doc = ""] # [doc = " See the [documentation on filtering with layers][filtering] for details."] # [doc = ""] # [doc = " [`Metadata`]: tracing_core::Metadata"] # [doc = " [`Filter`]: crate::layer::Filter"] # [doc = " [`Layer`]: crate::layer::Layer"] # [doc = " [plf]: crate::layer#per-layer-filtering"] # [doc = " [global]: crate::layer#global-filtering"] # [doc = " [filtering]: crate::layer#filtering-with-layers"] # [derive (Clone)] pub struct FilterFn < F = fn (& Metadata < '_ >) -> bool > { enabled : F , max_level_hint : Option < LevelFilter > , }
    };
}

FilterFn!();