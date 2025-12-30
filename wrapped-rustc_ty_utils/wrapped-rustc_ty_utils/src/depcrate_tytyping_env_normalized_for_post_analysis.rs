// Generated macro for typing_env_normalized_for_post_analysis (function)
macro_rules! Depcrate_tytyping_env_normalized_for_post_analysis {
() => {
// Module: crate::ty
// Provides: {"typing_env_normalized_for_post_analysis"}
// Dependencies: {}
fn typing_env_normalized_for_post_analysis (tcx : TyCtxt < '_ > , def_id : DefId) -> ty :: TypingEnv < '_ > { ty :: TypingEnv :: non_body_analysis (tcx , def_id) . with_post_analysis_normalized (tcx) }
};
}
