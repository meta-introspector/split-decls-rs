// Generated macro for impl_393 (impl)
macro_rules! Depcrate_os_macos_identityimpl_393 {
() => {
// Module: crate::os::macos::identity
// Provides: {"impl_393"}
// Dependencies: {}
impl SecIdentityExt for SecIdentity { fn with_certificate (keychains : & [SecKeychain] , certificate : & SecCertificate) -> Result < Self > { let keychains = CFArray :: from_CFTypes (keychains) ; unsafe { let mut identity = ptr :: null_mut () ; cvt (SecIdentityCreateWithCertificate (if ! keychains . is_empty () { keychains . as_CFTypeRef () } else { ptr :: null () } , certificate . as_concrete_TypeRef () , & mut identity ,)) ? ; Ok (Self :: wrap_under_create_rule (identity)) } } }
};
}
