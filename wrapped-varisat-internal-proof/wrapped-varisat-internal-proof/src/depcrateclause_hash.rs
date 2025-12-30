// Generated macro for clause_hash (function)
macro_rules! Depcrateclause_hash {
() => {
// Module: crate
// Provides: {"clause_hash"}
// Dependencies: {}
# [doc = " A fast hash function for clauses (or other *sets* of literals)."] # [doc = ""] # [doc = " This hash function interprets the given slice as a set and will not change when the input is"] # [doc = " permuted. It does not handle duplicated items."] pub fn clause_hash (lits : & [Lit]) -> ClauseHash { let mut hash = 0 ; for & lit in lits { hash ^= lit_hash (lit) ; } hash }
};
}
