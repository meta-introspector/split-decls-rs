// Generated macro for EncryptError (enum)
macro_rules! Depcrate_conn_unbufferedEncryptError {
() => {
// Module: crate::conn::unbuffered
// Provides: {"EncryptError"}
// Dependencies: {}
# [doc = " Errors that may arise when encrypting application data"] # [non_exhaustive] # [derive (Debug)] pub enum EncryptError { # [doc = " Provided buffer was too small"] InsufficientSize (InsufficientSizeError) , # [doc = " Encrypter has been exhausted"] EncryptExhausted , }
};
}
