// Generated macro for has_structural_eq_impl (function)
macro_rules! Depcrate_structural_matchhas_structural_eq_impl {
() => {
// Module: crate::structural_match
// Provides: {"has_structural_eq_impl"}
// Dependencies: {}
# [doc = " This method returns true if and only if `adt_ty` itself has been marked as"] # [doc = " eligible for structural-match: namely, if it implements"] # [doc = " `StructuralPartialEq` (which is injected by `#[derive(PartialEq)]`)."] # [doc = ""] # [doc = " Note that this does *not* recursively check if the substructure of `adt_ty`"] # [doc = " implements the trait."] fn has_structural_eq_impl < 'tcx > (tcx : TyCtxt < 'tcx > , adt_ty : Ty < 'tcx >) -> bool { let infcx = & tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let cause = ObligationCause :: dummy () ; let ocx = ObligationCtxt :: new (infcx) ; let structural_peq_def_id = infcx . tcx . require_lang_item (LangItem :: StructuralPeq , cause . span) ; ocx . register_bound (cause . clone () , ty :: ParamEnv :: empty () , adt_ty , structural_peq_def_id) ; ocx . select_all_or_error () . is_empty () }
};
}
