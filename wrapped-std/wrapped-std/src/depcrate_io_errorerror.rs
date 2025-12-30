// Generated macro for Error (struct)
macro_rules! Depcrate_io_errorError {
() => {
// Module: crate::io::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error type for I/O operations of the [`Read`], [`Write`], [`Seek`], and"] # [doc = " associated traits."] # [doc = ""] # [doc = " Errors mostly originate from the underlying OS, but custom instances of"] # [doc = " `Error` can be created with crafted error messages and a particular value of"] # [doc = " [`ErrorKind`]."] # [doc = ""] # [doc = " [`Read`]: crate::io::Read"] # [doc = " [`Write`]: crate::io::Write"] # [doc = " [`Seek`]: crate::io::Seek"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Error { repr : Repr , }
};
}
