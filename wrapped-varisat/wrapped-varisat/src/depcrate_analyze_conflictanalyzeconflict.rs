// Generated macro for AnalyzeConflict (struct)
macro_rules! Depcrate_analyze_conflictAnalyzeConflict {
() => {
// Module: crate::analyze_conflict
// Provides: {"AnalyzeConflict"}
// Dependencies: {}
# [doc = " Temporaries for conflict analysis"] # [derive (Default)] pub struct AnalyzeConflict { # [doc = " This is the learned clause after analysis finishes."] clause : Vec < Lit > , # [doc = " Number of literals in the current clause at the current level."] current_level_count : usize , # [doc = " Variables in the current clause."] var_flags : Vec < bool > , # [doc = " Entries to clean in `var_flags`."] to_clean : Vec < Var > , # [doc = " Clauses to bump."] involved : Vec < ClauseRef > , # [doc = " Hashes of all involved clauses needed to proof the minimized clause."] clause_hashes : Vec < ClauseHash > , # [doc = " Clause hashes paired with the trail depth of the propagated lit."] unordered_clause_hashes : Vec < (LitIdx , ClauseHash) > , # [doc = " Stack for recursive minimization."] stack : Vec < Lit > , }
};
}
