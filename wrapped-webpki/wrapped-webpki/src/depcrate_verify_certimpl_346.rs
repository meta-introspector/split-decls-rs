// Generated macro for impl_346 (impl)
macro_rules! Depcrate_verify_certimpl_346 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_346"}
// Dependencies: {}
impl fmt :: Debug for KeyPurposeId < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "KeyPurposeId(") ? ; let decoder = OidDecoder :: new (self . oid_value . as_slice_less_safe ()) ; for (i , part) in decoder . enumerate () { if i > 0 { write ! (f , ".") ? ; } write ! (f , "{part}") ? ; } write ! (f , ")") } }
};
}
