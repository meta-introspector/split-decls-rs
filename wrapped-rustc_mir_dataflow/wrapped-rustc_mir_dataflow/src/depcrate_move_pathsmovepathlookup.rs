// Generated macro for MovePathLookup (struct)
macro_rules! Depcrate_move_pathsMovePathLookup {
() => {
// Module: crate::move_paths
// Provides: {"MovePathLookup"}
// Dependencies: {}
# [doc = " Tables mapping from a place to its MovePathIndex."] # [derive (Debug)] pub struct MovePathLookup < 'tcx > { locals : IndexVec < Local , Option < MovePathIndex > > , # [doc = " projections are made from a base-place and a projection"] # [doc = " elem. The base-place will have a unique MovePathIndex; we use"] # [doc = " the latter as the index into the outer vector (narrowing"] # [doc = " subsequent search so that it is solely relative to that"] # [doc = " base-place). For the remaining lookup, we map the projection"] # [doc = " elem to the associated MovePathIndex."] projections : FxHashMap < (MovePathIndex , ProjectionKind) , MovePathIndex > , un_derefer : UnDerefer < 'tcx > , }
};
}
