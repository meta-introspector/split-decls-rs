// Generated macro for impl_302 (impl)
macro_rules! Depcrate_builder_matches_utilimpl_302 {
() => {
// Module: crate::builder::matches::util
// Provides: {"impl_302"}
// Dependencies: {}
impl < 'a , 'tcx > Builder < 'a , 'tcx > { # [doc = " Creates a false edge to `imaginary_target` and a real edge to"] # [doc = " real_target. If `imaginary_target` is none, or is the same as the real"] # [doc = " target, a Goto is generated instead to simplify the generated MIR."] pub (crate) fn false_edges (& mut self , from_block : BasicBlock , real_target : BasicBlock , imaginary_target : BasicBlock , source_info : SourceInfo ,) { if imaginary_target != real_target { self . cfg . terminate (from_block , source_info , TerminatorKind :: FalseEdge { real_target , imaginary_target } ,) ; } else { self . cfg . goto (from_block , source_info , real_target) } } }
};
}
