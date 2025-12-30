// Generated macro for impl_1060 (impl)
macro_rules! Depcrate_io_errorimpl_1060 {
() => {
// Module: crate::io::error
// Provides: {"impl_1060"}
// Dependencies: {}
# [stable (feature = "io_error_from_try_reserve" , since = "1.78.0")] impl From < alloc :: collections :: TryReserveError > for Error { # [doc = " Converts `TryReserveError` to an error with [`ErrorKind::OutOfMemory`]."] # [doc = ""] # [doc = " `TryReserveError` won't be available as the error `source()`,"] # [doc = " but this may change in the future."] fn from (_ : alloc :: collections :: TryReserveError) -> Error { ErrorKind :: OutOfMemory . into () } }
};
}
