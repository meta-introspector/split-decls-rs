macro_rules! deps {
    () => {
        MirPass!();
        Lint!();
    };
}

macro_rules! run_runtime_lowering_passes {
    () => {
        deps!();
        # [doc = " Returns the sequence of passes that lowers analysis to runtime MIR."] fn run_runtime_lowering_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let passes : & [& dyn MirPass < 'tcx >] = & [& add_call_guards :: CriticalCallEdges , & post_analysis_normalize :: PostAnalysisNormalize , & add_subtyping_projections :: Subtyper , & elaborate_drops :: ElaborateDrops , & Lint (check_call_recursion :: CheckDropRecursion) , & abort_unwinding_calls :: AbortUnwindingCalls , & add_moves_for_packed_drops :: AddMovesForPackedDrops , & add_retag :: AddRetag , & elaborate_box_derefs :: ElaborateBoxDerefs , & coroutine :: StateTransform , & Lint (known_panics_lint :: KnownPanicsLint) ,] ; pm :: run_passes_no_validate (tcx , body , passes , Some (MirPhase :: Runtime (RuntimePhase :: Initial))) ; }
    };
}

run_runtime_lowering_passes!()