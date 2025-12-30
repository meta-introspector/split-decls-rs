// Generated macro for matches_req (function)
macro_rules! Depcrate_evalmatches_req {
() => {
// Module: crate::eval
// Provides: {"matches_req"}
// Dependencies: {}
pub (crate) fn matches_req (req : & VersionReq , ver : & Version) -> bool { for cmp in & req . comparators { if ! matches_impl (cmp , ver) { return false ; } } if ver . pre . is_empty () { return true ; } for cmp in & req . comparators { if pre_is_compatible (cmp , ver) { return true ; } } false }
};
}
