// Generated macro for expand_or_pat (function)
macro_rules! Depcrate_rustcexpand_or_pat {
() => {
// Module: crate::rustc
// Provides: {"expand_or_pat"}
// Dependencies: {}
# [doc = " Recursively expand this pattern into its subpatterns. Only useful for or-patterns."] fn expand_or_pat < 'p , 'tcx > (pat : & 'p Pat < 'tcx >) -> Vec < & 'p Pat < 'tcx > > { fn expand < 'p , 'tcx > (pat : & 'p Pat < 'tcx > , vec : & mut Vec < & 'p Pat < 'tcx > >) { if let PatKind :: Or { pats } = & pat . kind { for pat in pats . iter () { expand (pat , vec) ; } } else { vec . push (pat) } } let mut pats = Vec :: new () ; expand (pat , & mut pats) ; pats }
};
}
