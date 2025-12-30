// Generated macro for provide (function)
macro_rules! Depcrate_needs_dropprovide {
() => {
// Module: crate::needs_drop
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { needs_drop_raw , needs_async_drop_raw , has_significant_drop_raw , adt_drop_tys , adt_async_drop_tys , adt_significant_drop_tys , list_significant_drop_tys , .. * providers } ; }
};
}
