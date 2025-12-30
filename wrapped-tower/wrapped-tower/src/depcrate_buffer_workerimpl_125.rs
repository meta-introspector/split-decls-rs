// Generated macro for impl_125 (impl)
macro_rules! Depcrate_buffer_workerimpl_125 {
() => {
// Module: crate::buffer::worker
// Provides: {"impl_125"}
// Dependencies: {}
impl Handle { pub (crate) fn get_error_on_closed (& self) -> crate :: BoxError { self . inner . lock () . unwrap () . as_ref () . map (| svc_err | svc_err . clone () . into ()) . unwrap_or_else (| | Closed :: new () . into ()) } }
};
}
