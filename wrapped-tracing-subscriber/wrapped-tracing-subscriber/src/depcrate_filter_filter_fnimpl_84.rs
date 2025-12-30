// Generated macro for impl_84 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_84 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_84"}
// Dependencies: {}
impl < S , F , R > Layer < S > for DynFilterFn < S , F , R > where F : Fn (& Metadata < '_ > , & Context < '_ , S >) -> bool + 'static , R : Fn (& 'static Metadata < 'static >) -> Interest + 'static , S : Subscriber , { fn enabled (& self , metadata : & Metadata < '_ > , cx : Context < '_ , S >) -> bool { self . is_enabled (metadata , & cx) } fn register_callsite (& self , metadata : & 'static Metadata < 'static >) -> Interest { self . is_callsite_enabled (metadata) } fn max_level_hint (& self) -> Option < LevelFilter > { self . max_level_hint } }
};
}
