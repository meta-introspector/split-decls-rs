// Generated macro for impl_84 (impl)
macro_rules! Depcrate_http_compat_conversionsimpl_84 {
() => {
// Module: crate::http_compat::conversions
// Provides: {"impl_84"}
// Dependencies: {}
impl From < & http :: Method > for Method { fn from (method : & http :: Method) -> Self { match method { & http :: Method :: GET => Method :: Get , & http :: Method :: POST => Method :: Post , & http :: Method :: PUT => Method :: Put , & http :: Method :: DELETE => Method :: Delete , & http :: Method :: PATCH => Method :: Patch , & http :: Method :: HEAD => Method :: Head , & http :: Method :: OPTIONS => Method :: Options , & http :: Method :: CONNECT => Method :: Connect , & http :: Method :: TRACE => Method :: Trace , other => Method :: Other (other . to_string ()) , } } }
};
}
