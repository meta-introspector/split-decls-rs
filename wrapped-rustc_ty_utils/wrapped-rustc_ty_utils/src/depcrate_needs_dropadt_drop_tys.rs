// Generated macro for adt_drop_tys (function)
macro_rules! Depcrate_needs_dropadt_drop_tys {
() => {
// Module: crate::needs_drop
// Provides: {"adt_drop_tys"}
// Dependencies: {}
fn adt_drop_tys < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId ,) -> Result < & 'tcx ty :: List < Ty < 'tcx > > , AlwaysRequiresDrop > { let adt_has_dtor = | adt_def : ty :: AdtDef < 'tcx > | adt_def . destructor (tcx) . map (| _ | DtorType :: Significant) ; drop_tys_helper (tcx , tcx . type_of (def_id) . instantiate_identity () , ty :: TypingEnv :: non_body_analysis (tcx , def_id) , adt_has_dtor , false , false ,) . collect :: < Result < Vec < _ > , _ > > () . map (| components | tcx . mk_type_list (& components)) }
};
}
