// Generated macro for SeekFrom (enum)
macro_rules! Depcrate_ioSeekFrom {
() => {
// Module: crate::io
// Provides: {"SeekFrom"}
// Dependencies: {}
# [doc = " Enumeration of possible methods to seek within an I/O object."] # [doc = ""] # [doc = " It is used by the [`Seek`] trait."] # [derive (Copy , PartialEq , Eq , Clone , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "SeekFrom")] pub enum SeekFrom { # [doc = " Sets the offset to the provided number of bytes."] # [stable (feature = "rust1" , since = "1.0.0")] Start (# [stable (feature = "rust1" , since = "1.0.0")] u64) , # [doc = " Sets the offset to the size of this object plus the specified number of"] # [doc = " bytes."] # [doc = ""] # [doc = " It is possible to seek beyond the end of an object, but it's an error to"] # [doc = " seek before byte 0."] # [stable (feature = "rust1" , since = "1.0.0")] End (# [stable (feature = "rust1" , since = "1.0.0")] i64) , # [doc = " Sets the offset to the current position plus the specified number of"] # [doc = " bytes."] # [doc = ""] # [doc = " It is possible to seek beyond the end of an object, but it's an error to"] # [doc = " seek before byte 0."] # [stable (feature = "rust1" , since = "1.0.0")] Current (# [stable (feature = "rust1" , since = "1.0.0")] i64) , }
};
}
