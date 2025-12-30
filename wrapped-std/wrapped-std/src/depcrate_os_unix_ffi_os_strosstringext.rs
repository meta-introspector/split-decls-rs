// Generated macro for OsStringExt (trait)
macro_rules! Depcrate_os_unix_ffi_os_strOsStringExt {
() => {
// Module: crate::os::unix::ffi::os_str
// Provides: {"OsStringExt"}
// Dependencies: {}
# [doc = " Platform-specific extensions to [`OsString`]."] # [doc = ""] # [doc = " This trait is sealed: it cannot be implemented outside the standard library."] # [doc = " This is so that future additional methods are not breaking changes."] # [stable (feature = "rust1" , since = "1.0.0")] pub trait OsStringExt : Sealed { # [doc = " Creates an [`OsString`] from a byte vector."] # [doc = ""] # [doc = " See the module documentation for an example."] # [stable (feature = "rust1" , since = "1.0.0")] fn from_vec (vec : Vec < u8 >) -> Self ; # [doc = " Yields the underlying byte vector of this [`OsString`]."] # [doc = ""] # [doc = " See the module documentation for an example."] # [stable (feature = "rust1" , since = "1.0.0")] fn into_vec (self) -> Vec < u8 > ; }
};
}
