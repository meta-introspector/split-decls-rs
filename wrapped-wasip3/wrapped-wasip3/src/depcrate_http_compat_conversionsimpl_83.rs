// Generated macro for impl_83 (impl)
macro_rules! Depcrate_http_compat_conversionsimpl_83 {
() => {
// Module: crate::http_compat::conversions
// Provides: {"impl_83"}
// Dependencies: {}
impl TryFrom < Method > for http :: Method { type Error = http :: method :: InvalidMethod ; fn try_from (method : Method) -> Result < Self , Self :: Error > { match method { Method :: Get => Ok (http :: Method :: GET) , Method :: Post => Ok (http :: Method :: POST) , Method :: Put => Ok (http :: Method :: PUT) , Method :: Delete => Ok (http :: Method :: DELETE) , Method :: Patch => Ok (http :: Method :: PATCH) , Method :: Head => Ok (http :: Method :: HEAD) , Method :: Options => Ok (http :: Method :: OPTIONS) , Method :: Connect => Ok (http :: Method :: CONNECT) , Method :: Trace => Ok (http :: Method :: TRACE) , Method :: Other (o) => http :: Method :: from_bytes (o . as_bytes ()) , } } }
};
}
