// Generated macro for impl_322 (impl)
macro_rules! Depcrate_verify_certimpl_322 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_322"}
// Dependencies: {}
impl < 'a > AsRef < [Option < Cert < 'a > >] > for Intermediates < 'a > { fn as_ref (& self) -> & [Option < Cert < 'a > >] { match self { Intermediates :: Owned { certs , used } => & certs [.. * used] , Intermediates :: Borrowed (certs) => certs , } } }
};
}
