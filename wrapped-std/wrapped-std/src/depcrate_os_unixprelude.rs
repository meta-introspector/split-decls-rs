// Generated macro for prelude (module)
macro_rules! Depcrate_os_unixprelude {
() => {
// Module: crate::os::unix
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " A prelude for conveniently writing platform-specific code."] # [doc = ""] # [doc = " Includes all extension traits, and some important type definitions."] # [stable (feature = "rust1" , since = "1.0.0")] pub mod prelude { # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: ffi :: { OsStrExt , OsStringExt } ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: DirEntryExt ; # [doc (no_inline)] # [stable (feature = "file_offset" , since = "1.15.0")] pub use super :: fs :: FileExt ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: { FileTypeExt , MetadataExt , OpenOptionsExt , PermissionsExt } ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ; # [doc (no_inline)] # [unstable (feature = "unix_send_signal" , issue = "141975")] pub use super :: process :: ChildExt ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: process :: { CommandExt , ExitStatusExt } ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: thread :: JoinHandleExt ; }
};
}
