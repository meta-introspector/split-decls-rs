// Generated macro for copy_canonical (function)
macro_rules! Depcrate_sorted_litscopy_canonical {
() => {
// Module: crate::sorted_lits
// Provides: {"copy_canonical"}
// Dependencies: {}
# [doc = " Sort literals, remove duplicates and check for tautologic clauses."] # [doc = ""] # [doc = " Return true if the clause is a tautology"] pub fn copy_canonical (target : & mut Vec < Lit > , src : & [Lit]) -> bool { target . clear () ; target . extend_from_slice (src) ; target . sort () ; target . dedup () ; let mut last = None ; target . iter () . any (| & lit | { let tautology = last == Some (! lit) ; last = Some (lit) ; tautology }) }
};
}
