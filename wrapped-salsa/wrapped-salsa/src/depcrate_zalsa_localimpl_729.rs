// Generated macro for impl_729 (impl)
macro_rules! Depcrate_zalsa_localimpl_729 {
() => {
// Module: crate::zalsa_local
// Provides: {"impl_729"}
// Dependencies: {}
impl Drop for QueryOrigin { fn drop (& mut self) { match self . kind { QueryOriginKind :: Derived | QueryOriginKind :: DerivedUntracked => { let input_outputs = unsafe { self . data . input_outputs } ; let length = self . metadata as usize ; let _input_outputs : Box < [QueryEdge] > = unsafe { Box :: from_raw (ptr :: slice_from_raw_parts_mut (input_outputs . as_ptr () , length ,)) } ; } QueryOriginKind :: FixpointInitial | QueryOriginKind :: Assigned => { } } } }
};
}
