// Generated macro for CheckerError (enum)
macro_rules! DepcrateCheckerError {
() => {
// Module: crate
// Provides: {"CheckerError"}
// Dependencies: {}
# [doc = " Possible errors while checking a varisat proof."] # [derive (Debug , Error)] # [non_exhaustive] pub enum CheckerError { # [error ("step {}: Unexpected end of proof file" , step)] ProofIncomplete { step : u64 } , # [error ("step {}: Error reading proof file: {}" , step , cause)] IoError { step : u64 , # [source] cause : io :: Error , } , # [error ("step {}: Could not parse proof step: {}" , step , cause)] ParseError { step : u64 , # [source] cause : Error , } , # [error ("step {}: Checking proof failed: {}" , step , msg)] CheckFailed { step : u64 , msg : String , debug_step : String , } , # [error ("Error in proof processor: {}" , cause)] ProofProcessorError { # [source] cause : Error , } , }
};
}
