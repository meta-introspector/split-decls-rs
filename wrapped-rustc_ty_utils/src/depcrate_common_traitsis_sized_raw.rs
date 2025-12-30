// Generated macro for is_sized_raw (function)
macro_rules! Depcrate_common_traitsis_sized_raw {
() => {
// Module: crate::common_traits
// Provides: {"is_sized_raw"}
// Dependencies: {}
fn is_sized_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Sized) }
};
}
