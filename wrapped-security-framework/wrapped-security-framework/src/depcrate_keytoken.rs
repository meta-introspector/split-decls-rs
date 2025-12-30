// Generated macro for Token (enum)
macro_rules! Depcrate_keyToken {
() => {
// Module: crate::key
// Provides: {"Token"}
// Dependencies: {}
# [doc = " Where to generate the key."] # [derive (Debug)] pub enum Token { # [doc = " Generate the key in software, compatible with all `KeyType`s."] Software , # [doc = " Generate the key in the Secure Enclave such that the private key is not"] # [doc = " extractable. Only compatible with `KeyType::ec()`."] SecureEnclave , }
};
}
