// Generated macro for IoSlice (struct)
macro_rules! Depcrate_ioIoSlice {
() => {
// Module: crate::io
// Provides: {"IoSlice"}
// Dependencies: {}
# [doc = " A buffer type used with `Write::write_vectored`."] # [doc = ""] # [doc = " It is semantically a wrapper around a `&[u8]`, but is guaranteed to be"] # [doc = " ABI compatible with the `iovec` type on Unix platforms and `WSABUF` on"] # [doc = " Windows."] # [stable (feature = "iovec" , since = "1.36.0")] # [derive (Copy , Clone)] # [repr (transparent)] pub struct IoSlice < 'a > (sys :: io :: IoSlice < 'a >) ;
};
}
