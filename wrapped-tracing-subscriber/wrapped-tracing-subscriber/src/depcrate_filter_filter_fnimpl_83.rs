// Generated macro for impl_83 (impl)
macro_rules! Depcrate_filter_filter_fnimpl_83 {
() => {
// Module: crate::filter::filter_fn
// Provides: {"impl_83"}
// Dependencies: {}
impl < S , F , R > DynFilterFn < S , F , R > where F : Fn (& Metadata < '_ > , & Context < '_ , S >) -> bool , R : Fn (& 'static Metadata < 'static >) -> Interest , { # [inline] fn is_enabled (& self , metadata : & Metadata < '_ > , cx : & Context < '_ , S >) -> bool { let enabled = (self . enabled) (metadata , cx) ; debug_assert ! (! enabled || is_below_max_level (& self . max_level_hint , metadata) , "DynFilterFn<{}> claimed it would only enable {:?} and below, \
            but it enabled metadata with the {:?} level\nmetadata={:#?}" , type_name ::< F > () , self . max_level_hint . unwrap () , metadata . level () , metadata ,) ; enabled } # [inline] fn is_callsite_enabled (& self , metadata : & 'static Metadata < 'static >) -> Interest { let interest = self . register_callsite . as_ref () . map (| callsite_enabled | callsite_enabled (metadata)) . unwrap_or_else (| | self . default_callsite_enabled (metadata)) ; debug_assert ! (interest . is_never () || is_below_max_level (& self . max_level_hint , metadata) , "DynFilterFn<{}, {}> claimed it was only interested in {:?} and below, \
            but it enabled metadata with the {:?} level\nmetadata={:#?}" , type_name ::< F > () , type_name ::< R > () , self . max_level_hint . unwrap () , metadata . level () , metadata ,) ; interest } }
};
}
