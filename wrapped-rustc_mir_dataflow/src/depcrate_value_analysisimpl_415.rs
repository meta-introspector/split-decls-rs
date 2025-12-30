// Generated macro for impl_415 (impl)
macro_rules! Depcrate_value_analysisimpl_415 {
() => {
// Module: crate::value_analysis
// Provides: {"impl_415"}
// Dependencies: {}
impl Iterator for Children < '_ , '_ > { type Item = PlaceIndex ; fn next (& mut self) -> Option < Self :: Item > { match self . next { Some (child) => { self . next = self . map . places [child] . next_sibling ; Some (child) } None => None , } } }
};
}
