// Generated macro for DeleteClauseProof (enum)
macro_rules! DepcrateDeleteClauseProof {
() => {
// Module: crate
// Provides: {"DeleteClauseProof"}
// Dependencies: {}
# [doc = " Justifications for a simple clause deletion."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum DeleteClauseProof { # [doc = " The clause is known to be redundant."] Redundant , # [doc = " The clause is irred and subsumed by the clause added in the previous step."] Simplified , # [doc = " The clause contains a true literal."] # [doc = ""] # [doc = " Also used to justify deletion of tautological clauses."] Satisfied , }
};
}
