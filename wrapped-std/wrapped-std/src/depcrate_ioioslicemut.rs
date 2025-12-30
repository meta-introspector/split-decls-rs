// Generated macro for IoSliceMut (struct)
macro_rules! Depcrate_ioIoSliceMut {
() => {
// Module: crate::io
// Provides: {"IoSliceMut"}
// Dependencies: {}
# [doc = " A buffer type used with `Read::read_vectored`."] # [doc = ""] # [doc = " It is semantically a wrapper around a `&mut [u8]`, but is guaranteed to be"] # [doc = " ABI compatible with the `iovec` type on Unix platforms and `WSABUF` on"] # [doc = " Windows."] # [stable (feature = "iovec" , since = "1.36.0")] # [repr (transparent)] pub struct IoSliceMut < 'a > (sys :: io :: IoSliceMut < 'a >) ;
};
}
