// Generated macro for DynFilterFn (struct)
macro_rules! Depcrate_filter_filter_fnDynFilterFn {
() => {
// Module: crate::filter::filter_fn
// Provides: {"DynFilterFn"}
// Dependencies: {}
# [doc = " A filter implemented by a closure or function pointer that"] # [doc = " determines whether a given span or event is enabled _dynamically_,"] # [doc = " potentially based on the current [span context]."] # [doc = ""] # [doc = " This type can be used for both [per-layer filtering][plf] (using its"] # [doc = " [`Filter`] implementation) and [global filtering][global] (using its"] # [doc = " [`Layer`] implementation)."] # [doc = ""] # [doc = " See the [documentation on filtering with layers][filtering] for details."] # [doc = ""] # [doc = " [span context]: crate::layer::Context"] # [doc = " [`Filter`]: crate::layer::Filter"] # [doc = " [`Layer`]: crate::layer::Layer"] # [doc = " [plf]: crate::layer#per-layer-filtering"] # [doc = " [global]: crate::layer#global-filtering"] # [doc = " [filtering]: crate::layer#filtering-with-layers"] pub struct DynFilterFn < S , F = fn (& Metadata < '_ > , & Context < '_ , S >) -> bool , R = fn (& 'static Metadata < 'static >) -> Interest , > { enabled : F , register_callsite : Option < R > , max_level_hint : Option < LevelFilter > , _s : PhantomData < fn (S) > , }
};
}
