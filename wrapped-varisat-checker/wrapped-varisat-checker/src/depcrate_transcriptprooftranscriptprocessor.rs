// Generated macro for ProofTranscriptProcessor (trait)
macro_rules! Depcrate_transcriptProofTranscriptProcessor {
() => {
// Module: crate::transcript
// Provides: {"ProofTranscriptProcessor"}
// Dependencies: {}
# [doc = " Implement to process transcript steps."] pub trait ProofTranscriptProcessor { # [doc = " Process a single proof transcript step."] fn process_step (& mut self , step : & ProofTranscriptStep) -> Result < () , Error > ; }
};
}
