// Generated macro for impl_80 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_80 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_80"}
// Dependencies: {}
impl < F > fmt :: Debug for FilterFn < F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FilterFn") . field ("enabled" , & format_args ! ("{}" , type_name ::< F > ())) . field ("max_level_hint" , & self . max_level_hint) . finish () } }
};
}
