// Generated macro for impl_1059 (impl)
macro_rules! Depcrate_io_errorimpl_1059 {
() => {
// Module: crate::io::error
// Provides: {"impl_1059"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl From < alloc :: ffi :: NulError > for Error { # [doc = " Converts a [`alloc::ffi::NulError`] into a [`Error`]."] fn from (_ : alloc :: ffi :: NulError) -> Error { const_error ! (ErrorKind :: InvalidInput , "data provided contains a nul byte") } }
};
}
