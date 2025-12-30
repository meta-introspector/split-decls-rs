// Generated macro for impl_143 (impl)
macro_rules! Depcrate_commonimpl_143 {
() => {
// Module: crate::common
// Provides: {"impl_143"}
// Dependencies: {}
impl Iterator for CodepointIter { type Item = Codepoint ; fn next (& mut self) -> Option < Codepoint > { if self . next > self . range . end . value () { return None ; } let current = self . next ; self . next += 1 ; Some (Codepoint :: from_u32 (current) . unwrap ()) } }
};
}
