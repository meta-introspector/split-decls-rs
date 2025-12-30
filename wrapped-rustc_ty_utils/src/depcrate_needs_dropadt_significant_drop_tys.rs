// Generated macro for adt_significant_drop_tys (function)
macro_rules! Depcrate_needs_dropadt_significant_drop_tys {
() => {
// Module: crate::needs_drop
// Provides: {"adt_significant_drop_tys"}
// Dependencies: {}
fn adt_significant_drop_tys (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Result < & ty :: List < Ty < '_ > > , AlwaysRequiresDrop > { drop_tys_helper (tcx , tcx . type_of (def_id) . instantiate_identity () , ty :: TypingEnv :: non_body_analysis (tcx , def_id) , adt_consider_insignificant_dtor (tcx) , true , false ,) . collect :: < Result < Vec < _ > , _ > > () . map (| components | tcx . mk_type_list (& components)) }
};
}
