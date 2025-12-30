// Generated macro for prelude (module)
macro_rules! Depcrate_os_windowsprelude {
() => {
// Module: crate::os::windows
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " A prelude for conveniently writing platform-specific code."] # [doc = ""] # [doc = " Includes all extension traits, and some important type definitions."] # [stable (feature = "rust1" , since = "1.0.0")] pub mod prelude { # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: ffi :: { OsStrExt , OsStringExt } ; # [doc (no_inline)] # [stable (feature = "file_offset" , since = "1.15.0")] pub use super :: fs :: FileExt ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: fs :: { MetadataExt , OpenOptionsExt } ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsHandle , AsSocket , BorrowedHandle , BorrowedSocket , FromRawHandle , FromRawSocket , HandleOrInvalid , IntoRawHandle , IntoRawSocket , OwnedHandle , OwnedSocket , } ; # [doc (no_inline)] # [stable (feature = "rust1" , since = "1.0.0")] pub use super :: io :: { AsRawHandle , AsRawSocket , RawHandle , RawSocket } ; }
};
}
