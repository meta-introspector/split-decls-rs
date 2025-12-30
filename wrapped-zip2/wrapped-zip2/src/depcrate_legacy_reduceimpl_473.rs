// Generated macro for impl_473 (impl)
macro_rules! Depcrate_legacy_reduceimpl_473 {
() => {
// Module: crate::legacy::reduce
// Provides: {"impl_473"}
// Dependencies: {}
impl < R : Read > ReduceDecoder < R > { pub fn new (inner : R , uncompressed_size : u64 , comp_factor : u8) -> Self { ReduceDecoder { compressed_reader : inner , uncompressed_size , stream_read : false , comp_factor , stream : Vec :: new () , read_pos : 0 , } } pub fn into_inner (self) -> R { self . compressed_reader } }
};
}
