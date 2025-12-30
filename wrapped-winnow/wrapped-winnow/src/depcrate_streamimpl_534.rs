// Generated macro for impl_534 (impl)
macro_rules! Depcrate_streamimpl_534 {
() => {
// Module: crate::stream
// Provides: {"impl_534"}
// Dependencies: {}
impl < I > Iterator for BitOffsets < I > where I : Stream < Token = u8 > + Clone , { type Item = (usize , bool) ; fn next (& mut self) -> Option < Self :: Item > { let b = next_bit (& mut self . i) ? ; let o = self . o ; self . o += 1 ; Some ((o , b)) } }
};
}
