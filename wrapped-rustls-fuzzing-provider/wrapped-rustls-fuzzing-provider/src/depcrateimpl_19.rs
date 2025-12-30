// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl ServerCredentialResolver for DummyCert { fn resolve (& self , client_hello : & ClientHello < '_ >) -> Result < SelectedCredential , Error > { self . 0 . signer (client_hello . signature_schemes ()) . ok_or (Error :: PeerIncompatible (PeerIncompatible :: NoSignatureSchemesInCommon ,)) } }
};
}
