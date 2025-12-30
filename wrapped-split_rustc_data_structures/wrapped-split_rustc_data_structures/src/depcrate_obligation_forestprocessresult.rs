// Generated macro for ProcessResult (enum)
macro_rules! Depcrate_obligation_forestProcessResult {
() => {
// Module: crate::obligation_forest
// Provides: {"ProcessResult"}
// Dependencies: {}
# [doc = " The result type used by `process_obligation`."] # [repr (C)] # [derive (Debug)] pub enum ProcessResult < O , E > { Unchanged , Changed (ThinVec < O >) , Error (E) , }
};
}
