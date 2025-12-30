// Generated macro for impl_586 (impl)
macro_rules! Depcrate_policyimpl_586 {
() => {
// Module: crate::policy
// Provides: {"impl_586"}
// Dependencies: {}
impl SecPolicy { # [doc = " Creates a `SecPolicy` for evaluating SSL certificate chains."] # [doc = ""] # [doc = " The side which you are evaluating should be provided (i.e. pass `SslSslProtocolSide::SERVER` if"] # [doc = " you are a client looking to validate a server's certificate chain)."] pub fn create_ssl (protocol_side : SslProtocolSide , hostname : Option < & str >) -> Self { let hostname = hostname . map (CFString :: new) ; let hostname = hostname . as_ref () . map (| s | s . as_concrete_TypeRef ()) . unwrap_or (ptr :: null_mut ()) ; let is_server = protocol_side == SslProtocolSide :: SERVER ; unsafe { let policy = SecPolicyCreateSSL (is_server . into () , hostname) ; Self :: wrap_under_create_rule (policy) } } # [doc = " Creates a `SecPolicy` for checking revocation of certificates."] # [doc = ""] # [doc = " If you do not specify this policy creating a `SecTrust` object, the system defaults"] # [doc = " will be used during evaluation."] pub fn create_revocation (options : RevocationPolicy) -> crate :: Result < Self > { let policy = unsafe { SecPolicyCreateRevocation (options . bits ()) } ; if policy . is_null () { Err (Error :: from_code (errSecParam)) } else { Ok (unsafe { Self :: wrap_under_create_rule (policy) }) } } # [doc = " Returns a policy object for the default X.509 policy."] # [must_use] pub fn create_x509 () -> Self { unsafe { let policy = SecPolicyCreateBasicX509 () ; Self :: wrap_under_create_rule (policy) } } }
};
}
