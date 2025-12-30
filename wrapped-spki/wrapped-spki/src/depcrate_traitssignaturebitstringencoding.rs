// Generated macro for SignatureBitStringEncoding (trait)
macro_rules! Depcrate_traitsSignatureBitStringEncoding {
() => {
// Module: crate::traits
// Provides: {"SignatureBitStringEncoding"}
// Dependencies: {}
# [doc = " Returns the `BitString` encoding of the signature."] # [doc = ""] # [doc = " X.509 and CSR structures require signatures to be BitString encoded."] # [cfg (feature = "alloc")] pub trait SignatureBitStringEncoding { # [doc = " `BitString` encoding for this signature."] fn to_bitstring (& self) -> der :: Result < BitString > ; }
};
}
