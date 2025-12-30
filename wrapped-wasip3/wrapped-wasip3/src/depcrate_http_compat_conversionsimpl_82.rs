// Generated macro for impl_82 (impl)
macro_rules! Depcrate_http_compat_conversionsimpl_82 {
() => {
// Module: crate::http_compat::conversions
// Provides: {"impl_82"}
// Dependencies: {}
impl From < & http :: uri :: Scheme > for Scheme { fn from (scheme : & http :: uri :: Scheme) -> Self { match scheme { s if s == & http :: uri :: Scheme :: HTTP => Scheme :: Http , s if s == & http :: uri :: Scheme :: HTTPS => Scheme :: Https , other => Scheme :: Other (other . to_string ()) , } } }
};
}
