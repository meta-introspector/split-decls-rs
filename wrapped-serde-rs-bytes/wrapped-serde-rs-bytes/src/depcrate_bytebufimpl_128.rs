// Generated macro for impl_128 (impl)
macro_rules! Depcrate_bytebufimpl_128 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_128"}
// Dependencies: {}
impl ByteBuf { # [doc = " Construct a new, empty `ByteBuf`."] pub fn new () -> Self { ByteBuf :: from (Vec :: new ()) } # [doc = " Construct a new, empty `ByteBuf` with the specified capacity."] pub fn with_capacity (cap : usize) -> Self { ByteBuf :: from (Vec :: with_capacity (cap)) } # [doc = " Wrap existing bytes in a `ByteBuf`."] pub fn from < T : Into < Vec < u8 > > > (bytes : T) -> Self { ByteBuf { bytes : bytes . into () , } } # [doc = " Unwrap the vector of byte underlying this `ByteBuf`."] pub fn into_vec (self) -> Vec < u8 > { self . bytes } # [allow (missing_docs)] pub fn into_boxed_bytes (self) -> Box < Bytes > { self . bytes . into_boxed_slice () . into () } # [doc (hidden)] pub fn into_boxed_slice (self) -> Box < [u8] > { self . bytes . into_boxed_slice () } # [doc (hidden)] # [allow (clippy :: should_implement_trait)] pub fn into_iter (self) -> < Vec < u8 > as IntoIterator > :: IntoIter { self . bytes . into_iter () } }
};
}
