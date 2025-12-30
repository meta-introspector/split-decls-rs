// Generated macro for SliceBackport (trait)
macro_rules! Depcrate_xxhash3SliceBackport {
() => {
// Module: crate::xxhash3
// Provides: {"SliceBackport"}
// Dependencies: {}
pub trait SliceBackport < T > { fn bp_as_chunks < const N : usize > (& self) -> (& [[T ; N]] , & [T]) ; fn bp_as_chunks_mut < const N : usize > (& mut self) -> (& mut [[T ; N]] , & mut [T]) ; fn bp_as_rchunks < const N : usize > (& self) -> (& [T] , & [[T ; N]]) ; }
};
}
