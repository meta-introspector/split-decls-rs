// Generated macro for Liveness (struct)
macro_rules! Depcrate_livenessLiveness {
() => {
// Module: crate::liveness
// Provides: {"Liveness"}
// Dependencies: {}
struct Liveness < 'a , 'tcx > { ir : & 'a mut IrMaps < 'tcx > , typeck_results : & 'a ty :: TypeckResults < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , closure_min_captures : Option < & 'tcx RootVariableMinCaptureList < 'tcx > > , successors : IndexVec < LiveNode , Option < LiveNode > > , rwu_table : rwu_table :: RWUTable , # [doc = " A live node representing a point of execution before closure entry &"] # [doc = " after closure exit. Used to calculate liveness of captured variables"] # [doc = " through calls to the same closure. Used for Fn & FnMut closures only."] closure_ln : LiveNode , # [doc = " A live node representing every 'exit' from the function, whether it be"] # [doc = " by explicit return, panic, or other means."] exit_ln : LiveNode , break_ln : HirIdMap < LiveNode > , cont_ln : HirIdMap < LiveNode > , }
};
}
