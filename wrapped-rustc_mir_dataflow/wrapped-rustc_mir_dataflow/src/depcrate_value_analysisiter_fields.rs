// Generated macro for iter_fields (function)
macro_rules! Depcrate_value_analysisiter_fields {
() => {
// Module: crate::value_analysis
// Provides: {"iter_fields"}
// Dependencies: {}
# [doc = " Invokes `f` on all direct fields of `ty`."] pub fn iter_fields < 'tcx > (ty : Ty < 'tcx > , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , mut f : impl FnMut (Option < VariantIdx > , FieldIdx , Ty < 'tcx >) ,) { match ty . kind () { ty :: Tuple (list) => { for (field , ty) in list . iter () . enumerate () { f (None , field . into () , ty) ; } } ty :: Adt (def , args) => { if def . is_union () { return ; } for (v_index , v_def) in def . variants () . iter_enumerated () { let variant = if def . is_struct () { None } else { Some (v_index) } ; for (f_index , f_def) in v_def . fields . iter () . enumerate () { let field_ty = f_def . ty (tcx , args) ; let field_ty = tcx . try_normalize_erasing_regions (typing_env , field_ty) . unwrap_or_else (| _ | tcx . erase_and_anonymize_regions (field_ty)) ; f (variant , f_index . into () , field_ty) ; } } } ty :: Closure (_ , args) => { iter_fields (args . as_closure () . tupled_upvars_ty () , tcx , typing_env , f) ; } ty :: Coroutine (_ , args) => { iter_fields (args . as_coroutine () . tupled_upvars_ty () , tcx , typing_env , f) ; } ty :: CoroutineClosure (_ , args) => { iter_fields (args . as_coroutine_closure () . tupled_upvars_ty () , tcx , typing_env , f) ; } _ => () , } }
};
}
