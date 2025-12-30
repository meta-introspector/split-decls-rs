// Generated macro for VerifyingKey (struct)
macro_rules! Depcrate_verifying_keyVerifyingKey {
() => {
// Module: crate::verifying_key
// Provides: {"VerifyingKey"}
// Dependencies: {}
# [doc = " A `VerifyingKey` is an SLH-DSA public key, allowing"] # [doc = " verification of signatures created with the corresponding"] # [doc = " `SigningKey`"] # [derive (Debug , PartialEq , Eq)] pub struct VerifyingKey < P : ParameterSet > { pub (crate) pk_seed : PkSeed < P :: N > , pub (crate) pk_root : Array < u8 , P :: N > , }
};
}
