// Generated macro for Sequence (enum)
macro_rules! Depcrate_encodingSequence {
() => {
// Module: crate::encoding
// Provides: {"Sequence"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug)] # [doc = " Sequences that a [`Matcher`] can produce"] pub enum Sequence < 'data > { # [doc = " Is encoded as a sequence for the decoder sequence execution."] # [doc = ""] # [doc = " First the literals will be copied to the decoded data,"] # [doc = " then `match_len` bytes are copied from `offset` bytes back in the decoded data"] Triple { literals : & 'data [u8] , offset : usize , match_len : usize , } , # [doc = " This is returned as the last sequence in a block"] # [doc = ""] # [doc = " These literals will just be copied at the end of the sequence execution by the decoder"] Literals { literals : & 'data [u8] } , }
};
}
