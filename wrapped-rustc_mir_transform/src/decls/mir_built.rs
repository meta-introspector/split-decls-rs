macro_rules! deps {
    () => {
        Optimizations!();
        Lint!();
    };
}

macro_rules! mir_built {
    () => {
        deps!();
        fn mir_built (tcx : TyCtxt < '_ > , def : LocalDefId) -> & Steal < Body < '_ > > { let mut body = build_mir (tcx , def) ; pass_manager :: dump_mir_for_phase_change (tcx , & body) ; pm :: run_passes (tcx , & mut body , & [& Lint (check_inline :: CheckForceInline) , & Lint (check_call_recursion :: CheckCallRecursion) , & Lint (check_inline_always_target_features :: CheckInlineAlwaysTargetFeature) , & Lint (check_packed_ref :: CheckPackedRef) , & Lint (check_const_item_mutation :: CheckConstItemMutation) , & Lint (function_item_references :: FunctionItemReferences) , & simplify :: SimplifyCfg :: Initial , & Lint (sanity_check :: SanityCheck) ,] , None , pm :: Optimizations :: Allowed ,) ; tcx . alloc_steal_mir (body) }
    };
}

mir_built!()