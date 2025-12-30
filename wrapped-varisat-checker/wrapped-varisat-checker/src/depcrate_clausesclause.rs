// Generated macro for Clause (struct)
macro_rules! Depcrate_clausesClause {
() => {
// Module: crate::clauses
// Provides: {"Clause"}
// Dependencies: {}
# [doc = " Literals and metadata for non-unit clauses."] pub struct Clause { # [doc = " LRAT clause id."] pub id : u64 , # [doc = " How often the clause is present as irred., red. clause."] # [doc = ""] # [doc = " For checking the formula is a multiset of clauses. This is necessary as the generating"] # [doc = " solver might not check for duplicated clauses."] ref_count : [u32 ; 2] , # [doc = " Clause's literals."] pub lits : ClauseLits , }
};
}
