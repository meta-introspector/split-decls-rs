// Generated macro for InvalidNameContext (struct)
macro_rules! Depcrate_errorInvalidNameContext {
() => {
// Module: crate::error
// Provides: {"InvalidNameContext"}
// Dependencies: {}
# [doc = " Additional context for the `CertNotValidForName` error variant."] # [doc = ""] # [doc = " The contents of this type depend on whether the `alloc` feature is enabled."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct InvalidNameContext { # [doc = " Expected server name."] # [cfg (feature = "alloc")] pub expected : ServerName < 'static > , # [doc = " The names presented in the end entity certificate."] # [doc = ""] # [doc = " These are the subject names as present in the leaf certificate and may contain DNS names"] # [doc = " with or without a wildcard label as well as IP address names."] # [cfg (feature = "alloc")] pub presented : Vec < String > , }
};
}
