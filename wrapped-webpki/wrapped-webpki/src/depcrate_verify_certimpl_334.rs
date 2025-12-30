// Generated macro for impl_334 (impl)
macro_rules! Depcrate_verify_certimpl_334 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_334"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Debug for EkuListDebug < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "[") ? ; for (i , part) in self . 0 . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "KeyPurposeId(") ? ; for (j , part) in part . iter () . enumerate () { if j > 0 { write ! (f , ".") ? ; } write ! (f , "{part}") ? ; } write ! (f , ")") ? ; } write ! (f , "]") } }
};
}
