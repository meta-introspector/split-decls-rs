// Generated macro for impl_102 (impl)
macro_rules! Depcrate_fileimpl_102 {
() => {
// Module: crate::file
// Provides: {"impl_102"}
// Dependencies: {}
# [cfg (windows)] impl < F : AsRawHandle > AsRawHandle for NamedTempFile < F > { # [inline] fn as_raw_handle (& self) -> RawHandle { self . as_file () . as_raw_handle () } }
};
}
