// Generated macro for impl_750 (impl)
macro_rules! Depcrate_itemsimpl_750 {
() => {
// Module: crate::items
// Provides: {"impl_750"}
// Dependencies: {}
impl WhereClauseOption { fn new (suppress_comma : bool , snuggle : WhereClauseSpace) -> WhereClauseOption { WhereClauseOption { suppress_comma , snuggle , allow_single_line : false , veto_single_line : false , } } fn snuggled (current : & str) -> WhereClauseOption { WhereClauseOption { suppress_comma : false , snuggle : if last_line_width (current) == 1 { WhereClauseSpace :: Space } else { WhereClauseSpace :: Newline } , allow_single_line : false , veto_single_line : false , } } fn suppress_comma (& mut self) { self . suppress_comma = true } fn allow_single_line (& mut self) { self . allow_single_line = true } fn snuggle (& mut self) { self . snuggle = WhereClauseSpace :: Space } fn veto_single_line (& mut self) { self . veto_single_line = true ; } }
};
}
