// Generated macro for impl_63 (impl)
macro_rules! Depcrate_expandimpl_63 {
() => {
// Module: crate::expand
// Provides: {"impl_63"}
// Dependencies: {}
impl VisitMut for AsyncTraitBlockReplacer < '_ > { fn visit_block_mut (& mut self , i : & mut Block) { if i == self . block { * i = self . patched_block . clone () ; } } }
};
}
