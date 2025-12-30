// Generated macro for macro_160 (macro)
macro_rules! Depcrate_filter_futuremacro_160 {
() => {
// Module: crate::filter::future
// Provides: {"macro_160"}
// Dependencies: {}
pin_project ! { # [project = StateProj] # [derive (Debug)] enum State < F , G > { # [doc = " Waiting for the predicate future"] Check { # [pin] check : F } , # [doc = " Waiting for the response future"] WaitResponse { # [pin] response : G } , } }
};
}
