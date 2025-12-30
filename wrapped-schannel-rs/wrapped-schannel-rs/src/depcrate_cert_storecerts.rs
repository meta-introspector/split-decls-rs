// Generated macro for Certs (struct)
macro_rules! Depcrate_cert_storeCerts {
() => {
// Module: crate::cert_store
// Provides: {"Certs"}
// Dependencies: {}
# [doc = " An iterator over the certificates contained in a `CertStore`, returned by"] # [doc = " `CertStore::iter`"] pub struct Certs < 'a > { store : & 'a CertStore , cur : Option < CertContext > , }
};
}
