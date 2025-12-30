// Generated macro for impl_73 (impl)
macro_rules! Depcrate_verification_windowsimpl_73 {
() => {
// Module: crate::verification::windows
// Provides: {"impl_73"}
// Dependencies: {}
impl CertChain { fn verify_chain_policy (& self , mut server_null_terminated : Vec < u16 > ,) -> Result < CERT_CHAIN_POLICY_STATUS , TlsError > { let mut extra_params = HTTPSPolicyCallbackData :: zeroed_with_size () ; extra_params . dwAuthType = AUTHTYPE_SERVER ; extra_params . pwszServerName = server_null_terminated . as_mut_ptr () ; let mut params = CERT_CHAIN_POLICY_PARA :: zeroed_with_size () ; params . dwFlags = CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS ; params . pvExtraPolicyPara = NonNull :: from (& mut extra_params) . cast :: < c_void > () . as_ptr () ; let mut status : MaybeUninit < CERT_CHAIN_POLICY_STATUS > = MaybeUninit :: uninit () ; let res = unsafe { CertVerifyCertificateChainPolicy (CERT_CHAIN_POLICY_SSL , self . inner . as_ptr () , & params , status . as_mut_ptr () ,) } ; if res != TRUE { return Err (TlsError :: General (String :: from ("TLS certificate verification was unavailable on the system!" ,))) ; } let status = unsafe { status . assume_init () } ; Ok (status) } }
};
}
