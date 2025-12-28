macro_rules! run_analysis_to_runtime_passes {
    () => {
        pub fn run_analysis_to_runtime_passes < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { assert ! (body . phase == MirPhase :: Analysis (AnalysisPhase :: Initial)) ; let did = body . source . def_id () ; debug ! ("analysis_mir_cleanup({:?})" , did) ; run_analysis_cleanup_passes (tcx , body) ; assert ! (body . phase == MirPhase :: Analysis (AnalysisPhase :: PostCleanup)) ; if check_consts :: post_drop_elaboration :: checking_enabled (& ConstCx :: new (tcx , body)) { pm :: run_passes (tcx , body , & [& remove_uninit_drops :: RemoveUninitDrops , & simplify :: SimplifyCfg :: RemoveFalseEdges , & Lint (post_drop_elaboration :: CheckLiveDrops) ,] , None , pm :: Optimizations :: Allowed ,) ; } debug ! ("runtime_mir_lowering({:?})" , did) ; run_runtime_lowering_passes (tcx , body) ; assert ! (body . phase == MirPhase :: Runtime (RuntimePhase :: Initial)) ; debug ! ("runtime_mir_cleanup({:?})" , did) ; run_runtime_cleanup_passes (tcx , body) ; assert ! (body . phase == MirPhase :: Runtime (RuntimePhase :: PostCleanup)) ; }
    };
}

run_analysis_to_runtime_passes!()