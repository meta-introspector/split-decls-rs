// Generated macro for run_analysis_cleanup_passes (function)
macro_rules! Depcraterun_analysis_cleanup_passes {
() => {
// Module: crate
// Provides: {"run_analysis_cleanup_passes"}
// Dependencies: {}
# [doc = " After this series of passes, no lifetime analysis based on borrowing can be done."] fn run_analysis_cleanup_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let passes : & [& dyn MirPass < 'tcx >] = & [& impossible_predicates :: ImpossiblePredicates , & cleanup_post_borrowck :: CleanupPostBorrowck , & remove_noop_landing_pads :: RemoveNoopLandingPads , & simplify :: SimplifyCfg :: PostAnalysis , & deref_separator :: Derefer ,] ; pm :: run_passes (tcx , body , passes , Some (MirPhase :: Analysis (AnalysisPhase :: PostCleanup)) , pm :: Optimizations :: Allowed ,) ; }
};
}
