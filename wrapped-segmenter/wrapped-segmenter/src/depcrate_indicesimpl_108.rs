// Generated macro for impl_108 (impl)
macro_rules! Depcrate_indicesimpl_108 {
() => {
// Module: crate::indices
// Provides: {"impl_108"}
// Dependencies: {}
impl Iterator for Latin1Indices < '_ > { type Item = (usize , u8) ; # [inline] fn next (& mut self) -> Option < (usize , u8) > { self . iter . get (self . front_offset) . map (| ch | { self . front_offset += 1 ; (self . front_offset - 1 , * ch) }) } }
};
}
