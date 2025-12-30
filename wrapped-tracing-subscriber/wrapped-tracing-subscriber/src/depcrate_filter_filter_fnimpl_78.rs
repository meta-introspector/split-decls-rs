// Generated macro for impl_78 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_78 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_78"}
// Dependencies: {}
impl < S , F > Layer < S > for FilterFn < F > where F : Fn (& Metadata < '_ >) -> bool + 'static , S : Subscriber , { fn enabled (& self , metadata : & Metadata < '_ > , _ : Context < '_ , S >) -> bool { self . is_enabled (metadata) } fn register_callsite (& self , metadata : & 'static Metadata < 'static >) -> Interest { self . is_callsite_enabled (metadata) } fn max_level_hint (& self) -> Option < LevelFilter > { self . max_level_hint } }
};
}
