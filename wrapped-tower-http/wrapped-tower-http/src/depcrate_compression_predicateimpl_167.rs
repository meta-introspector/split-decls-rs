// Generated macro for impl_167 (impl)
macro_rules! Depcrate_compression_predicateimpl_167 {
() => {
// Module: crate::compression::predicate
// Provides: {"impl_167"}
// Dependencies: {}
impl Predicate for SizeAbove { fn should_compress < B > (& self , response : & http :: Response < B >) -> bool where B : Body , { let content_size = response . body () . size_hint () . exact () . or_else (| | { response . headers () . get (header :: CONTENT_LENGTH) . and_then (| h | h . to_str () . ok ()) . and_then (| val | val . parse () . ok ()) }) ; match content_size { Some (size) => size >= (self . 0 as u64) , _ => true , } } }
};
}
