// Generated macro for impl_42 (impl)
macro_rules! Depcrate_hstringimpl_42 {
() => {
// Module: crate::hstring
// Provides: {"impl_42"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < & std :: ffi :: OsString > for HSTRING { fn from (value : & std :: ffi :: OsString) -> Self { value . as_os_str () . into () } }
};
}
