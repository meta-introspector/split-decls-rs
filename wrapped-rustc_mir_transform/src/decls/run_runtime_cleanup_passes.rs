macro_rules! deps {
    () => {
        Optimizations!();
        MirPass!();
    };
}

macro_rules! run_runtime_cleanup_passes {
    () => {
        deps!();
        # [doc = " Returns the sequence of passes that do the initial cleanup of runtime MIR."] fn run_runtime_cleanup_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let passes : & [& dyn MirPass < 'tcx >] = & [& lower_intrinsics :: LowerIntrinsics , & remove_place_mention :: RemovePlaceMention , & simplify :: SimplifyCfg :: PreOptimizations ,] ; pm :: run_passes (tcx , body , passes , Some (MirPhase :: Runtime (RuntimePhase :: PostCleanup)) , pm :: Optimizations :: Allowed ,) ; for decl in & mut body . local_decls { decl . local_info = ClearCrossCrate :: Clear ; } }
    };
}

run_runtime_cleanup_passes!();