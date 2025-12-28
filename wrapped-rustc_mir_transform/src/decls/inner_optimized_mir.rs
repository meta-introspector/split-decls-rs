macro_rules! inner_optimized_mir {
    () => {
        fn inner_optimized_mir (tcx : TyCtxt < '_ > , did : LocalDefId) -> Body < '_ > { if tcx . is_constructor (did . to_def_id ()) { return shim :: build_adt_ctor (tcx , did . to_def_id ()) ; } match tcx . hir_body_const_context (did) { Some (hir :: ConstContext :: ConstFn) => tcx . ensure_done () . mir_for_ctfe (did) , None => { } Some (other) => panic ! ("do not use `optimized_mir` for constants: {other:?}") , } debug ! ("about to call mir_drops_elaborated...") ; let body = tcx . mir_drops_elaborated_and_const_checked (did) . steal () ; let mut body = remap_mir_for_const_eval_select (tcx , body , hir :: Constness :: NotConst) ; if body . tainted_by_errors . is_some () { return body ; } mentioned_items :: MentionedItems . run_pass (tcx , & mut body) ; if let TerminatorKind :: Unreachable = body . basic_blocks [START_BLOCK] . terminator () . kind && body . basic_blocks [START_BLOCK] . statements . is_empty () { return body ; } run_optimization_passes (tcx , & mut body) ; body }
    };
}

inner_optimized_mir!()