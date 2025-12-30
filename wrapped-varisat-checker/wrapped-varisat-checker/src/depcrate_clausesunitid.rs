// Generated macro for UnitId (enum)
macro_rules! Depcrate_clausesUnitId {
() => {
// Module: crate::clauses
// Provides: {"UnitId"}
// Dependencies: {}
# [doc = " Identifies the origin of a unit clause."] # [derive (Copy , Clone , Debug)] pub enum UnitId { Global (u64) , TracePos (usize) , InClause , }
};
}
