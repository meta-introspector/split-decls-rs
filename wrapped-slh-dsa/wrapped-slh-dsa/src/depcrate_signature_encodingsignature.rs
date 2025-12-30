// Generated macro for Signature (struct)
macro_rules! Depcrate_signature_encodingSignature {
() => {
// Module: crate::signature_encoding
// Provides: {"Signature"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq)] # [doc = " A parsed SLH-DSA signature for a given parameter set"] # [doc = ""] # [doc = " Note that this is a large stack-allocated value and may overflow the stack on"] # [doc = " small devices. The stack representation consumes `P::SigLen` bytes"] # [doc = ""] # [doc = " There are no invariants maintained by this struct - every field is a hash value"] pub struct Signature < P : ParameterSet > { pub (crate) randomizer : Array < u8 , P :: N > , pub (crate) fors_sig : ForsSignature < P > , pub (crate) ht_sig : HypertreeSig < P > , }
};
}
