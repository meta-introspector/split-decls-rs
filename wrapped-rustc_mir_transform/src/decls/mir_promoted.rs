macro_rules! deps {
    () => {
        Optimizations!();
    };
}

macro_rules! mir_promoted {
    () => {
        deps!();
        # [doc = " Compute the main MIR body and the list of MIR bodies of the promoteds."] fn mir_promoted (tcx : TyCtxt < '_ > , def : LocalDefId ,) -> (& Steal < Body < '_ > > , & Steal < IndexVec < Promoted , Body < '_ > > >) { let const_qualifs = match tcx . def_kind (def) { DefKind :: Fn | DefKind :: AssocFn | DefKind :: Closure if tcx . constness (def) == hir :: Constness :: Const || tcx . is_const_default_method (def . to_def_id ()) => { tcx . mir_const_qualif (def) } DefKind :: AssocConst | DefKind :: Const | DefKind :: Static { .. } | DefKind :: InlineConst | DefKind :: AnonConst => tcx . mir_const_qualif (def) , _ => ConstQualifs :: default () , } ; tcx . ensure_done () . has_ffi_unwind_calls (def) ; if tcx . needs_coroutine_by_move_body_def_id (def . to_def_id ()) { tcx . ensure_done () . coroutine_by_move_body_def_id (def) ; } let mut body = tcx . mir_built (def) . steal () ; if let Some (error_reported) = const_qualifs . tainted_by_errors { body . tainted_by_errors = Some (error_reported) ; } RequiredConstsVisitor :: compute_required_consts (& mut body) ; let promote_pass = promote_consts :: PromoteTemps :: default () ; pm :: run_passes (tcx , & mut body , & [& promote_pass , & simplify :: SimplifyCfg :: PromoteConsts , & coverage :: InstrumentCoverage] , Some (MirPhase :: Analysis (AnalysisPhase :: Initial)) , pm :: Optimizations :: Allowed ,) ; lint_tail_expr_drop_order :: run_lint (tcx , def , & body) ; let promoted = promote_pass . promoted_fragments . into_inner () ; (tcx . alloc_steal_mir (body) , tcx . alloc_steal_promoted (promoted)) }
    };
}

mir_promoted!();