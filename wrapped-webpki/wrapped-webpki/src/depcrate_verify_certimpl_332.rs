// Generated macro for impl_332 (impl)
macro_rules! Depcrate_verify_certimpl_332 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_332"}
// Dependencies: {}
impl fmt :: Debug for RequiredEkuNotFoundContext { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = f . debug_struct ("RequiredEkuNotFoundContext") ; # [cfg (feature = "alloc")] builder . field ("required" , match & self . required . inner { EkuValidationMode :: Required (inner) => inner , EkuValidationMode :: RequiredIfPresent (inner) => inner , } ,) ; # [cfg (feature = "alloc")] builder . field ("present" , & EkuListDebug (& self . present)) ; builder . finish () } }
};
}
