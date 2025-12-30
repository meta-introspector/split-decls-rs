// Generated macro for impl_307 (impl)
macro_rules! Depcrate_walkimpl_307 {
() => {
// Module: crate::walk
// Provides: {"impl_307"}
// Dependencies: {}
impl < I : Interner > Iterator for TypeWalker < I > { type Item = I :: GenericArg ; fn next (& mut self) -> Option < I :: GenericArg > { debug ! ("next(): stack={:?}" , self . stack) ; loop { let next = self . stack . pop () ? ; self . last_subtree = self . stack . len () ; if self . visited . insert (next) { push_inner :: < I > (& mut self . stack , next) ; debug ! ("next: stack={:?}" , self . stack) ; return Some (next) ; } } } }
};
}
