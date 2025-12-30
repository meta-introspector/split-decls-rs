// Generated macro for Clauses (struct)
macro_rules! Depcrate_clausesClauses {
() => {
// Module: crate::clauses
// Provides: {"Clauses"}
// Dependencies: {}
# [doc = " Checker clause storage."] # [derive (Default)] pub struct Clauses { # [doc = " Next clause id to use."] pub next_clause_id : u64 , # [doc = " Literal storage for clauses,"] pub literal_buffer : Vec < Lit > , # [doc = " Number of literals in the buffer which are from deleted clauses."] garbage_size : usize , # [doc = " Stores all known non-unit clauses indexed by their hash."] pub clauses : HashMap < ClauseHash , SmallVec < [Clause ; 1] > > , # [doc = " Stores known unit clauses and propagations during a clause check."] pub unit_clauses : Vec < Option < UnitClause > > , # [doc = " This stores a conflict of input unit clauses."] # [doc = ""] # [doc = " Our representation for unit clauses doesn't support conflicting units so this is used as a"] # [doc = " workaround."] pub unit_conflict : Option < [u64 ; 2] > , }
};
}
