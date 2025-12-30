// Generated macro for SelfChecker (trait)
macro_rules! Depcrate_internalSelfChecker {
() => {
// Module: crate::internal
// Provides: {"SelfChecker"}
// Dependencies: {}
# [doc = " Varisat internal interface used for on-the-fly checking."] # [doc = ""] # [doc = " This should only be used within other Varisat crates. It is not considered part of the public"] # [doc = " API and may change at any time."] pub trait SelfChecker { fn self_check_step (& mut self , step : ProofStep) -> Result < () , CheckerError > ; fn self_check_delayed_steps (& mut self) -> Result < () , CheckerError > ; }
};
}
