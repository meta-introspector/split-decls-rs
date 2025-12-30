// Generated macro for impl_99 (impl)
macro_rules! Depcrate_fileimpl_99 {
() => {
// Module: crate::file
// Provides: {"impl_99"}
// Dependencies: {}
# [cfg (any (unix , target_os = "wasi"))] impl < F : AsFd > AsFd for NamedTempFile < F > { fn as_fd (& self) -> BorrowedFd < '_ > { self . as_file () . as_fd () } }
};
}
