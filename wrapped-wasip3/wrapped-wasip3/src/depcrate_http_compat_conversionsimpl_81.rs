// Generated macro for impl_81 (impl)
macro_rules! Depcrate_http_compat_conversionsimpl_81 {
() => {
// Module: crate::http_compat::conversions
// Provides: {"impl_81"}
// Dependencies: {}
impl TryFrom < Scheme > for http :: uri :: Scheme { type Error = http :: uri :: InvalidUri ; fn try_from (scheme : Scheme) -> Result < Self , Self :: Error > { match scheme { Scheme :: Http => Ok (http :: uri :: Scheme :: HTTP) , Scheme :: Https => Ok (http :: uri :: Scheme :: HTTPS) , Scheme :: Other (s) => s . parse () , } } }
};
}
