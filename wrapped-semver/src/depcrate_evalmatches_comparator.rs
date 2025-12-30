// Generated macro for matches_comparator (function)
macro_rules! Depcrate_evalmatches_comparator {
() => {
// Module: crate::eval
// Provides: {"matches_comparator"}
// Dependencies: {}
pub (crate) fn matches_comparator (cmp : & Comparator , ver : & Version) -> bool { matches_impl (cmp , ver) && (ver . pre . is_empty () || pre_is_compatible (cmp , ver)) }
};
}
