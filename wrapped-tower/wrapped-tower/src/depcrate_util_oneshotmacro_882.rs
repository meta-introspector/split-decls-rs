// Generated macro for macro_882 (macro)
macro_rules! Depcrate_util_oneshotmacro_882 {
() => {
// Module: crate::util::oneshot
// Provides: {"macro_882"}
// Dependencies: {}
pin_project ! { # [project = StateProj] enum State < S : Service < Req >, Req > { NotReady { svc : S , req : Option < Req >, } , Called { # [pin] fut : S :: Future , } , Done , } }
};
}
