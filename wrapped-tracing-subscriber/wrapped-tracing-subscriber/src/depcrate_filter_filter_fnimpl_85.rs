// Generated macro for impl_85 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_85 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_85"}
// Dependencies: {}
impl < S , F , R > fmt :: Debug for DynFilterFn < S , F , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("DynFilterFn") ; s . field ("enabled" , & format_args ! ("{}" , type_name ::< F > ())) ; if self . register_callsite . is_some () { s . field ("register_callsite" , & format_args ! ("Some({})" , type_name ::< R > ()) ,) ; } else { s . field ("register_callsite" , & format_args ! ("None")) ; } s . field ("max_level_hint" , & self . max_level_hint) . finish () } }
};
}
