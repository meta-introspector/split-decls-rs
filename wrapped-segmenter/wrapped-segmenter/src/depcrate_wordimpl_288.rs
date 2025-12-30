// Generated macro for impl_288 (impl)
macro_rules! Depcrate_wordimpl_288 {
() => {
// Module: crate::word
// Provides: {"impl_288"}
// Dependencies: {}
impl < Y : RuleBreakType > Iterator for WordBreakIteratorWithWordType < '_ , '_ , Y > { type Item = (usize , WordType) ; fn next (& mut self) -> Option < Self :: Item > { let ret = self . 0 . next () ? ; Some ((ret , self . 0 . 0 . word_type ())) } }
};
}
