// Generated macro for EncodeError (enum)
macro_rules! Depcrate_conn_unbufferedEncodeError {
() => {
// Module: crate::conn::unbuffered
// Provides: {"EncodeError"}
// Dependencies: {}
# [doc = " Errors that may arise when encoding a handshake record"] # [non_exhaustive] # [derive (Debug)] pub enum EncodeError { # [doc = " Provided buffer was too small"] InsufficientSize (InsufficientSizeError) , # [doc = " The handshake record has already been encoded; do not call `encode` again"] AlreadyEncoded , }
};
}
