// Generated macro for FindSlice (trait)
macro_rules! Depcrate_streamFindSlice {
() => {
// Module: crate::stream
// Provides: {"FindSlice"}
// Dependencies: {}
# [doc = " Look for a slice in self"] pub trait FindSlice < T > { # [doc = " Returns the offset of the slice if it is found"] fn find_slice (& self , substr : T) -> Option < core :: ops :: Range < usize > > ; }
};
}
