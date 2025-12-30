// Generated macro for impl_101 (impl)
macro_rules! Depcrate_fileimpl_101 {
() => {
// Module: crate::file
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (windows)] impl < F : AsHandle > AsHandle for NamedTempFile < F > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { self . as_file () . as_handle () } }
};
}
