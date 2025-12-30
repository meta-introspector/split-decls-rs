// Generated macro for ExecuteSequencesError (enum)
macro_rules! Depcrate_decoding_errorsExecuteSequencesError {
() => {
// Module: crate::decoding::errors
// Provides: {"ExecuteSequencesError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum ExecuteSequencesError { DecodebufferError (DecodeBufferError) , NotEnoughBytesForSequence { wanted : usize , have : usize } , ZeroOffset , }
};
}
