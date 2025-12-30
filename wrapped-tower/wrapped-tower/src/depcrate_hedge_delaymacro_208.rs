// Generated macro for macro_208 (macro)
macro_rules! Depcrate_hedge_delaymacro_208 {
() => {
// Module: crate::hedge::delay
// Provides: {"macro_208"}
// Dependencies: {}
pin_project ! { # [project = StateProj] # [derive (Debug)] enum State < Request , F > { Delaying { # [pin] delay : tokio :: time :: Sleep , req : Option < Request >, } , Called { # [pin] fut : F , } , } }
};
}
