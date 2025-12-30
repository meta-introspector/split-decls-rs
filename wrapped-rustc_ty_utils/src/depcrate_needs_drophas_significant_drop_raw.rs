// Generated macro for has_significant_drop_raw (function)
macro_rules! Depcrate_needs_drophas_significant_drop_raw {
() => {
// Module: crate::needs_drop
// Provides: {"has_significant_drop_raw"}
// Dependencies: {}
fn has_significant_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { let res = drop_tys_helper (tcx , query . value , query . typing_env , adt_consider_insignificant_dtor (tcx) , true , false ,) . filter (filter_array_elements (tcx , query . typing_env)) . next () . is_some () ; debug ! ("has_significant_drop_raw({:?}) = {:?}" , query , res) ; res }
};
}
