macro_rules! deps {
    () => {
        CostChecker!();
        Optimizations!();
    };
}

macro_rules! cross_crate_inlinable {
    () => {
        deps!();
        fn cross_crate_inlinable (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let codegen_fn_attrs = tcx . codegen_fn_attrs (def_id) ; if codegen_fn_attrs . contains_extern_indicator () { return false ; } match tcx . def_kind (def_id) { DefKind :: Ctor (..) | DefKind :: Closure | DefKind :: SyntheticCoroutineBody => return true , DefKind :: Fn | DefKind :: AssocFn => { } _ => return false , } if tcx . sess . opts . unstable_opts . cross_crate_inline_threshold == InliningThreshold :: Always { return true ; } if tcx . has_attr (def_id , sym :: rustc_intrinsic) { return true ; } match codegen_fn_attrs . inline { InlineAttr :: Never => return false , InlineAttr :: Hint | InlineAttr :: Always | InlineAttr :: Force { .. } => return true , _ => { } } if tcx . sess . opts . unstable_opts . hint_mostly_unused { return true ; } let sig = tcx . fn_sig (def_id) . instantiate_identity () ; for ty in sig . inputs () . skip_binder () . iter () . chain (std :: iter :: once (& sig . output () . skip_binder ())) { if ty == & tcx . types . f16 || ty == & tcx . types . f128 { return true ; } } if tcx . sess . opts . incremental . is_some () { return false ; } let inliner_will_run = pm :: should_run_pass (tcx , & inline :: Inline , pm :: Optimizations :: Allowed) || inline :: ForceInline :: should_run_pass_for_callee (tcx , def_id . to_def_id ()) ; if matches ! (tcx . sess . opts . optimize , OptLevel :: No) && ! inliner_will_run { return false ; } if ! tcx . is_mir_available (def_id) { return false ; } let threshold = match tcx . sess . opts . unstable_opts . cross_crate_inline_threshold { InliningThreshold :: Always => return true , InliningThreshold :: Sometimes (threshold) => threshold , InliningThreshold :: Never => return false , } ; let mir = tcx . optimized_mir (def_id) ; let mut checker = CostChecker { tcx , callee_body : mir , calls : 0 , statements : 0 , landing_pads : 0 , resumes : 0 } ; checker . visit_body (mir) ; checker . calls == 0 && checker . resumes == 0 && checker . landing_pads == 0 && checker . statements <= threshold }
    };
}

cross_crate_inlinable!()