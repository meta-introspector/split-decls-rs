// Generated macro for is_unpin_raw (function)
macro_rules! Depcrate_common_traitsis_unpin_raw {
() => {
// Module: crate::common_traits
// Provides: {"is_unpin_raw"}
// Dependencies: {}
fn is_unpin_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Unpin) }
};
}
