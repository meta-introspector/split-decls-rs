// Generated macro for Take (struct)
macro_rules! Depcrate_ioTake {
() => {
// Module: crate::io
// Provides: {"Take"}
// Dependencies: {}
# [doc = " Reader adapter which limits the bytes read from an underlying reader."] # [doc = ""] # [doc = " This struct is generally created by calling [`take`] on a reader."] # [doc = " Please see the documentation of [`take`] for more details."] # [doc = ""] # [doc = " [`take`]: Read::take"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct Take < T > { inner : T , len : u64 , limit : u64 , }
};
}
