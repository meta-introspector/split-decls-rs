// Generated macro for SigningKey (struct)
macro_rules! Depcrate_signing_keySigningKey {
() => {
// Module: crate::signing_key
// Provides: {"SigningKey"}
// Dependencies: {}
# [doc = " A `SigningKey` allows signing messages with a fixed parameter set"] # [derive (Clone , PartialEq , Eq , Debug)] pub struct SigningKey < P : ParameterSet > { pub (crate) sk_seed : SkSeed < P :: N > , pub (crate) sk_prf : SkPrf < P :: N > , pub (crate) verifying_key : VerifyingKey < P > , }
};
}
