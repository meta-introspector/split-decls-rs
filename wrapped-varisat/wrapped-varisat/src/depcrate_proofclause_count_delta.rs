// Generated macro for clause_count_delta (function)
macro_rules! Depcrate_proofclause_count_delta {
() => {
// Module: crate::proof
// Provides: {"clause_count_delta"}
// Dependencies: {}
# [doc = " Number of added or removed clauses."] pub fn clause_count_delta (step : & ProofStep) -> isize { match step { ProofStep :: AddClause { clause } | ProofStep :: AtClause { clause , .. } => { if clause . len () > 1 { 1 } else { 0 } } ProofStep :: DeleteClause { clause , .. } => { if clause . len () > 1 { - 1 } else { 0 } } ProofStep :: SolverVarName { .. } | ProofStep :: UserVarName { .. } | ProofStep :: DeleteVar { .. } | ProofStep :: ChangeSamplingMode { .. } | ProofStep :: UnitClauses { .. } | ProofStep :: ChangeHashBits { .. } | ProofStep :: Model { .. } | ProofStep :: Assumptions { .. } | ProofStep :: FailedAssumptions { .. } | ProofStep :: End => 0 , } }
};
}
