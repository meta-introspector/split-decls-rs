// Generated macro for impl_145 (impl)
macro_rules! Depcrate_astimpl_145 {
() => {
// Module: crate::ast
// Provides: {"impl_145"}
// Dependencies: {}
impl < N : AstNode > Iterator for AstChildren < N > { type Item = N ; fn next (& mut self) -> Option < N > { self . inner . find_map (N :: cast) } }
};
}
