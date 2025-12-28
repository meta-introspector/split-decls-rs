macro_rules! check_callees_abi {
    () => {
        fn check_callees_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & mir :: Body < 'tcx >) { for (bb , _data) in traversal :: mono_reachable (body , tcx , instance) { let terminator = body . basic_blocks [bb] . terminator () ; match terminator . kind { mir :: TerminatorKind :: Call { ref func , ref fn_span , .. } | mir :: TerminatorKind :: TailCall { ref func , ref fn_span , .. } => { let callee_ty = func . ty (body , tcx) ; let callee_ty = instance . instantiate_mir_and_normalize_erasing_regions (tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (callee_ty) ,) ; check_call_site_abi (tcx , callee_ty , body . source . instance , | | { let loc = Location { block : bb , statement_index : body . basic_blocks [bb] . statements . len () , } ; (* fn_span , body . source_info (loc) . scope . lint_root (& body . source_scopes) . unwrap_or (CRATE_HIR_ID) ,) }) ; } _ => { } } } }
    };
}

check_callees_abi!();