// Generated macro for impl_28 (impl)
macro_rules! Depcrate_linesimpl_28 {
() => {
// Module: crate::lines
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'b > Iterator for LineIter < 'b > { type Item = & 'b [u8] ; fn next (& mut self) -> Option < & 'b [u8] > { self . stepper . next_match (self . bytes) . map (| m | & self . bytes [m]) } }
};
}
