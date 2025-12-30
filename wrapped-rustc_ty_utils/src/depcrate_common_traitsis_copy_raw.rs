// Generated macro for is_copy_raw (function)
macro_rules! Depcrate_common_traitsis_copy_raw {
() => {
// Module: crate::common_traits
// Provides: {"is_copy_raw"}
// Dependencies: {}
fn is_copy_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Copy) }
};
}
