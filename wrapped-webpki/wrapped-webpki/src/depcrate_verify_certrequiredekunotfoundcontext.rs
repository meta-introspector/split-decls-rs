// Generated macro for RequiredEkuNotFoundContext (struct)
macro_rules! Depcrate_verify_certRequiredEkuNotFoundContext {
() => {
// Module: crate::verify_cert
// Provides: {"RequiredEkuNotFoundContext"}
// Dependencies: {}
# [doc = " Additional context for the `RequiredEkuNotFoundContext` error variant."] # [doc = ""] # [doc = " The contents of this type depend on whether the `alloc` feature is enabled."] # [derive (Clone , PartialEq , Eq)] pub struct RequiredEkuNotFoundContext { # [doc = " The required ExtendedKeyUsage."] # [cfg (feature = "alloc")] pub required : ExtendedKeyUsage , # [doc = " The ExtendedKeyUsage OIDs present in the certificate."] # [cfg (feature = "alloc")] pub present : Vec < Vec < usize > > , }
};
}
