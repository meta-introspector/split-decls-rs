// Generated macro for impl_345 (impl)
macro_rules! Depcrate_verify_certimpl_345 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_345"}
// Dependencies: {}
impl < 'a > KeyPurposeId < 'a > { # [doc = " Construct a new [`KeyPurposeId`]."] # [doc = ""] # [doc = " `oid` is the OBJECT IDENTIFIER in bytes."] pub const fn new (oid : & 'a [u8]) -> Self { Self { oid_value : untrusted :: Input :: from (oid) , } } # [doc = " Yield the OID value as a sequence of `usize` components."] # [cfg (feature = "alloc")] pub fn to_decoded_oid (& self) -> Vec < usize > { OidDecoder :: new (self . oid_value . as_slice_less_safe ()) . collect () } }
};
}
