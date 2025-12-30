// Generated macro for Bytes (struct)
macro_rules! Depcrate_ioBytes {
() => {
// Module: crate::io
// Provides: {"Bytes"}
// Dependencies: {}
# [doc = " An iterator over `u8` values of a reader."] # [doc = ""] # [doc = " This struct is generally created by calling [`bytes`] on a reader."] # [doc = " Please see the documentation of [`bytes`] for more details."] # [doc = ""] # [doc = " [`bytes`]: Read::bytes"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct Bytes < R > { inner : R , }
};
}
