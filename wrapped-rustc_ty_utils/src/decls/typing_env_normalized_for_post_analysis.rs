macro_rules! typing_env_normalized_for_post_analysis {
    () => {
        fn typing_env_normalized_for_post_analysis (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: TypingEnv < '_ > { ty :: TypingEnv :: non_body_analysis (tcx , def_id) . with_post_analysis_normalized (tcx) }
    };
}

typing_env_normalized_for_post_analysis!()