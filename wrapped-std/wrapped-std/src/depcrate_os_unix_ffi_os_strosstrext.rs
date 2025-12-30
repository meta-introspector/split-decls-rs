// Generated macro for OsStrExt (trait)
macro_rules! Depcrate_os_unix_ffi_os_strOsStrExt {
() => {
// Module: crate::os::unix::ffi::os_str
// Provides: {"OsStrExt"}
// Dependencies: {}
# [doc = " Platform-specific extensions to [`OsStr`]."] # [doc = ""] # [doc = " This trait is sealed: it cannot be implemented outside the standard library."] # [doc = " This is so that future additional methods are not breaking changes."] # [stable (feature = "rust1" , since = "1.0.0")] pub trait OsStrExt : Sealed { # [stable (feature = "rust1" , since = "1.0.0")] # [doc = " Creates an [`OsStr`] from a byte slice."] # [doc = ""] # [doc = " See the module documentation for an example."] fn from_bytes (slice : & [u8]) -> & Self ; # [doc = " Gets the underlying byte view of the [`OsStr`] slice."] # [doc = ""] # [doc = " See the module documentation for an example."] # [stable (feature = "rust1" , since = "1.0.0")] fn as_bytes (& self) -> & [u8] ; }
};
}
