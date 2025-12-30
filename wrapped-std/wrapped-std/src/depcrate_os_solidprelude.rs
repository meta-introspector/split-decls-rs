// Generated macro for prelude (module)
macro_rules! Depcrate_os_solidprelude {
() => {
// Module: crate::os::solid
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " A prelude for conveniently writing platform-specific code."] # [doc = ""] # [doc = " Includes all extension traits, and some important type definitions."] # [stable (feature = "rust1" , since = "1.0.0")] pub mod prelude { # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: ffi :: { OsStrExt , OsStringExt } ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ; }
};
}
