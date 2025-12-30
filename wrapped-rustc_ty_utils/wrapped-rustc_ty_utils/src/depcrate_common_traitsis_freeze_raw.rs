// Generated macro for is_freeze_raw (function)
macro_rules! Depcrate_common_traitsis_freeze_raw {
() => {
// Module: crate::common_traits
// Provides: {"is_freeze_raw"}
// Dependencies: {}
fn is_freeze_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Freeze) }
};
}
