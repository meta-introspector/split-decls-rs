// Generated macro for impl_496 (impl)
macro_rules! Depcrate_legacy_shrinkimpl_496 {
() => {
// Module: crate::legacy::shrink
// Provides: {"impl_496"}
// Dependencies: {}
impl < R : Read > ShrinkDecoder < R > { pub fn new (inner : R , uncompressed_size : u64) -> Self { Self { compressed_reader : inner , uncompressed_size , stream_read : false , stream : Vec :: new () , read_pos : 0 , } } pub fn into_inner (self) -> R { self . compressed_reader } }
};
}
