// Generated macro for impl_162 (impl)
macro_rules! Depcrate_identityimpl_162 {
() => {
// Module: crate::identity
// Provides: {"impl_162"}
// Dependencies: {}
impl SecIdentity { # [doc = " Returns the certificate corresponding to this identity."] pub fn certificate (& self) -> Result < SecCertificate > { unsafe { let mut certificate = ptr :: null_mut () ; cvt (SecIdentityCopyCertificate (self . 0 , & mut certificate)) ? ; Ok (SecCertificate :: wrap_under_create_rule (certificate)) } } # [doc = " Returns the private key corresponding to this identity."] pub fn private_key (& self) -> Result < SecKey > { unsafe { let mut key = ptr :: null_mut () ; cvt (SecIdentityCopyPrivateKey (self . 0 , & mut key)) ? ; Ok (SecKey :: wrap_under_create_rule (key)) } } # [doc = " Translates to `SecItemDelete`, passing in the `SecIdentityRef`"] pub fn delete (& self) -> Result < () , Error > { let query = CFMutableDictionary :: from_CFType_pairs (& [(unsafe { kSecValueRef } . to_void () , self . to_void () ,)]) ; cvt (unsafe { SecItemDelete (query . as_concrete_TypeRef ()) }) } }
};
}
