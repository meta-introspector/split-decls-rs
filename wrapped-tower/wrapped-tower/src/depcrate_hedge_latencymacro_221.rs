// Generated macro for macro_221 (macro)
macro_rules! Depcrate_hedge_latencymacro_221 {
() => {
// Module: crate::hedge::latency
// Provides: {"macro_221"}
// Dependencies: {}
pin_project ! { # [derive (Debug)] pub struct ResponseFuture < R , F > { start : Instant , rec : R , # [pin] inner : F , } }
};
}
