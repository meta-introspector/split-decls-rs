// Generated macro for RevocationOptions (struct)
macro_rules! Depcrate_crlRevocationOptions {
() => {
// Module: crate::crl
// Provides: {"RevocationOptions"}
// Dependencies: {}
# [doc = " Describes how revocation checking is performed, if at all. Can be constructed with a"] # [doc = " [RevocationOptionsBuilder] instance."] # [derive (Debug , Copy , Clone)] pub struct RevocationOptions < 'a > { pub (crate) crls : & 'a [& 'a CertRevocationList < 'a >] , pub (crate) depth : RevocationCheckDepth , pub (crate) status_policy : UnknownStatusPolicy , pub (crate) expiration_policy : ExpirationPolicy , }
};
}
