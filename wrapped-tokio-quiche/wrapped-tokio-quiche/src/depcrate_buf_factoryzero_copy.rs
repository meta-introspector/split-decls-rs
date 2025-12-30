// Generated macro for zero_copy (module)
macro_rules! Depcrate_buf_factoryzero_copy {
() => {
// Module: crate::buf_factory
// Provides: {"zero_copy"}
// Dependencies: {}
# [cfg (feature = "zero-copy")] mod zero_copy { use super :: PooledBuf ; use quiche :: BufSplit ; # [doc = " A pooled, splittable byte buffer for zero-copy [`quiche`] calls."] # [derive (Clone , Debug)] pub struct QuicheBuf { inner : triomphe :: Arc < PooledBuf > , start : usize , end : usize , } impl QuicheBuf { pub (crate) fn new (inner : PooledBuf) -> Self { QuicheBuf { start : 0 , end : inner . len () , inner : triomphe :: Arc :: new (inner) , } } } impl AsRef < [u8] > for QuicheBuf { fn as_ref (& self) -> & [u8] { & self . inner [self . start .. self . end] } } impl BufSplit for QuicheBuf { fn split_at (& mut self , at : usize) -> Self { assert ! (self . start + at <= self . end) ; let split = QuicheBuf { inner : self . inner . clone () , start : self . start + at , end : self . end , } ; self . end = self . start + at ; split } fn try_add_prefix (& mut self , prefix : & [u8]) -> bool { if self . start != 0 { return false ; } if let Some (unique) = triomphe :: Arc :: get_mut (& mut self . inner) { if unique . add_prefix (prefix) { self . end += prefix . len () ; return true ; } } false } } impl quiche :: BufFactory for super :: BufFactory { type Buf = QuicheBuf ; fn buf_from_slice (buf : & [u8]) -> Self :: Buf { QuicheBuf :: new (Self :: buf_from_slice (buf)) } } }
};
}
