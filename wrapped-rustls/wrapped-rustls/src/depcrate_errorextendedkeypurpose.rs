// Generated macro for ExtendedKeyPurpose (enum)
macro_rules! Depcrate_errorExtendedKeyPurpose {
() => {
// Module: crate::error
// Provides: {"ExtendedKeyPurpose"}
// Dependencies: {}
# [doc = " Extended Key Usage (EKU) purpose values."] # [doc = ""] # [doc = " These are usually represented as OID values in the certificate's extension (if present), but"] # [doc = " we represent the values that are most relevant to rustls as named enum variants."] # [non_exhaustive] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ExtendedKeyPurpose { # [doc = " Client authentication"] ClientAuth , # [doc = " Server authentication"] ServerAuth , # [doc = " Other EKU values"] # [doc = ""] # [doc = " Represented here as a `Vec<usize>` for human readability."] Other (Vec < usize >) , }
};
}
