// Generated macro for AnonymousClientPolicy (enum)
macro_rules! Depcrate_webpki_client_verifierAnonymousClientPolicy {
() => {
// Module: crate::webpki::client_verifier
// Provides: {"AnonymousClientPolicy"}
// Dependencies: {}
# [doc = " Controls how the [WebPkiClientVerifier] handles anonymous clients."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum AnonymousClientPolicy { # [doc = " Clients that do not present a client certificate are allowed."] Allow , # [doc = " Clients that do not present a client certificate are denied."] Deny , }
};
}
