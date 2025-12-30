// Generated macro for macro_566 (macro)
macro_rules! Depcrate_retry_futuremacro_566 {
() => {
// Module: crate::retry::future
// Provides: {"macro_566"}
// Dependencies: {}
pin_project ! { # [project = StateProj] # [derive (Debug)] enum State < F , P > { Called { # [pin] future : F } , Waiting { # [pin] waiting : P } , Retrying , } }
};
}
