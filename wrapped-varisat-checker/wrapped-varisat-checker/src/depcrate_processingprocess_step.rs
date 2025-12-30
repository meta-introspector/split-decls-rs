// Generated macro for process_step (function)
macro_rules! Depcrate_processingprocess_step {
() => {
// Module: crate::processing
// Provides: {"process_step"}
// Dependencies: {}
# [doc = " Process a single step"] pub fn process_step < 'a , 'b > (mut ctx : partial ! (Context <'a >, mut ProcessingP <'a >, VariablesP) , step : & CheckedProofStep < 'b > ,) -> Result < () , CheckerError > { let (processing , mut ctx) = ctx . split_part_mut (ProcessingP) ; processing . step (step , CheckerData (ctx . borrow ())) }
};
}
