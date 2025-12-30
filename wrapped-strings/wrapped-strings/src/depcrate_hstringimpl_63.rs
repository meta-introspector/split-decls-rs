// Generated macro for impl_63 (impl)
macro_rules! Depcrate_hstringimpl_63 {
() => {
// Module: crate::hstring
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (feature = "std")] impl PartialEq < std :: ffi :: OsStr > for HSTRING { fn eq (& self , other : & std :: ffi :: OsStr) -> bool { self . iter () . copied () . eq (std :: os :: windows :: ffi :: OsStrExt :: encode_wide (other)) } }
};
}
