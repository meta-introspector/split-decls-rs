macro_rules! ThirBuildCx {
    () => {
        # [doc = " Context for lowering HIR to THIR for a single function body (or other kind of body)."] struct ThirBuildCx < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " The THIR data that this context is building."] thir : Thir < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , region_scope_tree : & 'tcx region :: ScopeTree , typeck_results : & 'tcx ty :: TypeckResults < 'tcx > , rvalue_scopes : & 'tcx RvalueScopes , # [doc = " False to indicate that adjustments should not be applied. Only used for `custom_mir`"] apply_adjustments : bool , # [doc = " The `DefId` of the owner of this body."] body_owner : DefId , }
    };
}

ThirBuildCx!();