// Generated macro for impl_161 (impl)
macro_rules! Depcrate_identityimpl_161 {
() => {
// Module: crate::identity
// Provides: {"impl_161"}
// Dependencies: {}
impl fmt :: Debug for SecIdentity { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("SecIdentity") ; if let Ok (cert) = self . certificate () { builder . field ("certificate" , & cert) ; } if let Ok (key) = self . private_key () { builder . field ("private_key" , & key) ; } builder . finish () } }
};
}
