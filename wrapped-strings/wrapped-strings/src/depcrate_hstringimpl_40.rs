// Generated macro for impl_40 (impl)
macro_rules! Depcrate_hstringimpl_40 {
() => {
// Module: crate::hstring
// Provides: {"impl_40"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < & std :: ffi :: OsStr > for HSTRING { fn from (value : & std :: ffi :: OsStr) -> Self { unsafe { Self :: from_wide_iter (std :: os :: windows :: ffi :: OsStrExt :: encode_wide (value) , value . len () ,) } } }
};
}
