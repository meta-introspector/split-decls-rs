// Generated macro for ParseCtxt (struct)
macro_rules! Depcrate_builder_customParseCtxt {
() => {
// Module: crate::builder::custom
// Provides: {"ParseCtxt"}
// Dependencies: {}
struct ParseCtxt < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , thir : & 'a Thir < 'tcx > , source_scope : SourceScope , body : & 'a mut Body < 'tcx > , local_map : FxHashMap < LocalVarId , Local > , block_map : FxHashMap < LocalVarId , BasicBlock > , }
};
}
