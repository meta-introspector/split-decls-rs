// Generated macro for impl_66 (impl)
macro_rules! Depcrate_processingimpl_66 {
() => {
// Module: crate::processing
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > Processing < 'a > { # [doc = " Process a single step"] pub fn step < 'b > (& mut self , step : & CheckedProofStep < 'b > , data : CheckerData ,) -> Result < () , CheckerError > { for processor in self . processors . iter_mut () { if let Err (err) = processor . process_step (step , data) { return Err (CheckerError :: ProofProcessorError { cause : err }) ; } } if ! self . transcript_processors . is_empty () { if let Some (transcript_step) = self . transcript . transcript_step (step , data) { for processor in self . transcript_processors . iter_mut () { if let Err (err) = processor . process_step (& transcript_step) { return Err (CheckerError :: ProofProcessorError { cause : err }) ; } } } } Ok (()) } }
};
}
