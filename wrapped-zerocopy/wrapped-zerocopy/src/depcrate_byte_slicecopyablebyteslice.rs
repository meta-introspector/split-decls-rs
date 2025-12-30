// Generated macro for CopyableByteSlice (trait)
macro_rules! Depcrate_byte_sliceCopyableByteSlice {
() => {
// Module: crate::byte_slice
// Provides: {"CopyableByteSlice"}
// Dependencies: {}
# [doc = " A [`ByteSlice`] which can be copied without violating dereference stability."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If `B: CopyableByteSlice`, then the dereference stability properties"] # [doc = " required by [`ByteSlice`] (see that trait's safety documentation) do not"] # [doc = " only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also"] # [doc = " hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by"] # [doc = " copying `b`."] pub unsafe trait CopyableByteSlice : ByteSlice + Copy + CloneableByteSlice { }
};
}
