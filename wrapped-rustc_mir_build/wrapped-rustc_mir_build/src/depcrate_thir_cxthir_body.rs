// Generated macro for thir_body (function)
macro_rules! Depcrate_thir_cxthir_body {
() => {
// Module: crate::thir::cx
// Provides: {"thir_body"}
// Dependencies: {}
# [doc = " Query implementation for [`TyCtxt::thir_body`]."] pub (crate) fn thir_body (tcx : TyCtxt < '_ > , owner_def : LocalDefId ,) -> Result < (& Steal < Thir < '_ > > , ExprId) , ErrorGuaranteed > { let body = tcx . hir_body_owned_by (owner_def) ; let mut cx = ThirBuildCx :: new (tcx , owner_def) ; if let Some (reported) = cx . typeck_results . tainted_by_errors { return Err (reported) ; } let owner_id = tcx . local_def_id_to_hir_id (owner_def) ; if let Some (fn_decl) = tcx . hir_fn_decl_by_hir_id (owner_id) { let closure_env_param = cx . closure_env_param (owner_def , owner_id) ; let explicit_params = cx . explicit_params (owner_id , fn_decl , & body) ; cx . thir . params = closure_env_param . into_iter () . chain (explicit_params) . collect () ; if tcx . is_coroutine (owner_def . to_def_id ()) && body . params . is_empty () { cx . thir . params . push (Param { ty : tcx . types . unit , pat : None , ty_span : None , self_kind : None , hir_id : None , }) ; } } let expr = cx . mirror_expr (body . value) ; Ok ((tcx . alloc_steal_thir (cx . thir) , expr)) }
};
}
