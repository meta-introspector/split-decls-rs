// Generated macro for CrlBuilder (struct)
macro_rules! Depcrate_request_builderCrlBuilder {
() => {
// Module: crate::request::builder
// Provides: {"CrlBuilder"}
// Dependencies: {}
# [doc = " X.509 CRL builder"] pub struct CrlBuilder < P = certificate :: Rfc5280 > where P : certificate :: Profile , { tbs : TbsCertList < P > , }
};
}
