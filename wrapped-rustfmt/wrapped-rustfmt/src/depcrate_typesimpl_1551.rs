// Generated macro for impl_1551 (impl)
macro_rules! Depcrate_typesimpl_1551 {
() => {
// Module: crate::types
// Provides: {"impl_1551"}
// Dependencies: {}
impl Rewrite for ast :: PolyTraitRef { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { let (binder , shape) = if let Some (lifetime_str) = rewrite_bound_params (context , shape , & self . bound_generic_params) { let extra_offset = lifetime_str . len () + 6 ; let shape = shape . offset_left (extra_offset , self . span) ? ; (format ! ("for<{lifetime_str}> ") , shape) } else { (String :: new () , shape) } ; let ast :: TraitBoundModifiers { constness , asyncness , polarity , } = self . modifiers ; let mut constness = constness . as_str () . to_string () ; if ! constness . is_empty () { constness . push (' ') ; } let mut asyncness = asyncness . as_str () . to_string () ; if ! asyncness . is_empty () { asyncness . push (' ') ; } let polarity = polarity . as_str () ; let shape = shape . offset_left (constness . len () + polarity . len () , self . span) ? ; let path_str = self . trait_ref . rewrite_result (context , shape) ? ; Ok (format ! ("{binder}{constness}{asyncness}{polarity}{path_str}")) } }
};
}
