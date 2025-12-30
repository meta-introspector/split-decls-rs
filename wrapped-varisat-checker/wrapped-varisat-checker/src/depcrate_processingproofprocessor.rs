// Generated macro for ProofProcessor (trait)
macro_rules! Depcrate_processingProofProcessor {
() => {
// Module: crate::processing
// Provides: {"ProofProcessor"}
// Dependencies: {}
# [doc = " Implement to process proof steps."] pub trait ProofProcessor { fn process_step (& mut self , step : & CheckedProofStep , data : CheckerData) -> Result < () , Error > ; }
};
}
