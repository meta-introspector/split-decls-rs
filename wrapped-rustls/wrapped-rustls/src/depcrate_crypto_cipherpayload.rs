// Generated macro for Payload (enum)
macro_rules! Depcrate_crypto_cipherPayload {
() => {
// Module: crate::crypto::cipher
// Provides: {"Payload"}
// Dependencies: {}
# [doc = " An externally length'd payload"] # [non_exhaustive] # [derive (Clone , Eq , PartialEq)] pub enum Payload < 'a > { # [doc = " Borrowed payload"] Borrowed (& 'a [u8]) , # [doc = " Owned payload"] Owned (Vec < u8 >) , }
};
}
