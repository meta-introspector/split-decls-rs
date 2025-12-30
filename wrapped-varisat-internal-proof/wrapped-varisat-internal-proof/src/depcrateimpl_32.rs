// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > ProofStep < 'a > { # [doc = " Does this proof step use clause hashes?"] pub fn contains_hashes (& self) -> bool { match self { ProofStep :: AtClause { .. } | ProofStep :: UnitClauses { .. } | ProofStep :: FailedAssumptions { .. } => true , ProofStep :: SolverVarName { .. } | ProofStep :: UserVarName { .. } | ProofStep :: DeleteVar { .. } | ProofStep :: ChangeSamplingMode { .. } | ProofStep :: AddClause { .. } | ProofStep :: DeleteClause { .. } | ProofStep :: ChangeHashBits { .. } | ProofStep :: Model { .. } | ProofStep :: Assumptions { .. } | ProofStep :: End => false , } } }
};
}
