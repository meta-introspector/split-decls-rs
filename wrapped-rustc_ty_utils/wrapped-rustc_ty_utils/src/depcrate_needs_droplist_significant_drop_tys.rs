// Generated macro for list_significant_drop_tys (function)
macro_rules! Depcrate_needs_droplist_significant_drop_tys {
() => {
// Module: crate::needs_drop
// Provides: {"list_significant_drop_tys"}
// Dependencies: {}
# [instrument (level = "debug" , skip (tcx) , ret)] fn list_significant_drop_tys < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> & 'tcx ty :: List < Ty < 'tcx > > { tcx . mk_type_list (& drop_tys_helper (tcx , key . value , key . typing_env , adt_consider_insignificant_dtor (tcx) , true , true ,) . filter_map (| res | res . ok ()) . collect :: < Vec < _ > > () ,) }
};
}
