// Generated macro for is_async_drop_raw (function)
macro_rules! Depcrate_common_traitsis_async_drop_raw {
() => {
// Module: crate::common_traits
// Provides: {"is_async_drop_raw"}
// Dependencies: {}
fn is_async_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { is_item_raw (tcx , query , LangItem :: AsyncDrop) }
};
}
