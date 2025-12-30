// Generated macro for impl_337 (impl)
macro_rules! Depcrate_verify_certimpl_337 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_337"}
// Dependencies: {}
impl ExtendedKeyUsageValidator for ExtendedKeyUsage { fn validate (& self , iter : KeyPurposeIdIter < '_ , '_ >) -> Result < () , Error > { let mut empty = true ; # [cfg (feature = "alloc")] let mut present = Vec :: new () ; for id in iter { empty = false ; let id = id ? ; if self . inner . id () == id { return Ok (()) ; } # [cfg (feature = "alloc")] present . push (id . to_decoded_oid ()) ; } match (empty , self . inner) { (true , EkuValidationMode :: RequiredIfPresent (_)) => Ok (()) , _ => Err (Error :: RequiredEkuNotFound (RequiredEkuNotFoundContext { # [cfg (feature = "alloc")] required : Self { inner : self . inner } , # [cfg (feature = "alloc")] present , })) , } } }
};
}
