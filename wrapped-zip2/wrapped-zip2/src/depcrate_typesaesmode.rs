// Generated macro for AesMode (enum)
macro_rules! Depcrate_typesAesMode {
() => {
// Module: crate::types
// Provides: {"AesMode"}
// Dependencies: {}
# [doc = " AES variant used."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (fuzzing , derive (arbitrary :: Arbitrary))] # [repr (u8)] pub enum AesMode { # [doc = " 128-bit AES encryption."] Aes128 = 0x01 , # [doc = " 192-bit AES encryption."] Aes192 = 0x02 , # [doc = " 256-bit AES encryption."] Aes256 = 0x03 , }
};
}
