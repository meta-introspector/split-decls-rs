// Generated macro for thir_abstract_const (function)
macro_rules! Depcrate_conststhir_abstract_const {
() => {
// Module: crate::consts
// Provides: {"thir_abstract_const"}
// Dependencies: {}
# [doc = " Builds an abstract const, do not use this directly, but use `AbstractConst::new` instead."] fn thir_abstract_const < 'tcx > (tcx : TyCtxt < 'tcx > , def : LocalDefId ,) -> Result < Option < ty :: EarlyBinder < 'tcx , ty :: Const < 'tcx > > > , ErrorGuaranteed > { if ! tcx . features () . generic_const_exprs () { return Ok (None) ; } match tcx . def_kind (def) { DefKind :: AnonConst | DefKind :: InlineConst => () , _ => return Ok (None) , } let body = tcx . thir_body (def) ? ; let (body , body_id) = (& * body . 0 . borrow () , body . 1) ; let mut is_poly_vis = IsThirPolymorphic { is_poly : false , thir : body } ; visit :: walk_expr (& mut is_poly_vis , & body [body_id]) ; if ! is_poly_vis . is_poly { return Ok (None) ; } let root_span = body . exprs [body_id] . span ; Ok (Some (ty :: EarlyBinder :: bind (recurse_build (tcx , body , body_id , root_span) ?))) }
};
}
