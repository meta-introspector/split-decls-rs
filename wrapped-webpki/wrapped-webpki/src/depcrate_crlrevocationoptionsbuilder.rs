// Generated macro for RevocationOptionsBuilder (struct)
macro_rules! Depcrate_crlRevocationOptionsBuilder {
() => {
// Module: crate::crl
// Provides: {"RevocationOptionsBuilder"}
// Dependencies: {}
# [doc = " Builds a RevocationOptions instance to control how revocation checking is performed."] # [derive (Debug , Copy , Clone)] pub struct RevocationOptionsBuilder < 'a > { crls : & 'a [& 'a CertRevocationList < 'a >] , depth : RevocationCheckDepth , status_policy : UnknownStatusPolicy , expiration_policy : ExpirationPolicy , }
};
}
