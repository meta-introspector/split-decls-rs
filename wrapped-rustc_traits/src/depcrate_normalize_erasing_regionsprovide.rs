// Generated macro for provide (function)
macro_rules! Depcrate_normalize_erasing_regionsprovide {
() => {
// Module: crate::normalize_erasing_regions
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (p : & mut Providers) { * p = Providers { try_normalize_generic_arg_after_erasing_regions : | tcx , goal | { debug ! ("try_normalize_generic_arg_after_erasing_regions(goal={:#?}" , goal) ; try_normalize_after_erasing_regions (tcx , goal) } , .. * p } ; }
};
}
