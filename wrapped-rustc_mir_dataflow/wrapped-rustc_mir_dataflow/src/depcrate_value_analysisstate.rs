// Generated macro for State (enum)
macro_rules! Depcrate_value_analysisState {
() => {
// Module: crate::value_analysis
// Provides: {"State"}
// Dependencies: {}
# [doc = " Dataflow state."] # [doc = ""] # [doc = " Every instance specifies a lattice that represents the possible values of a single tracked"] # [doc = " place. If we call this lattice `V` and set of tracked places `P`, then a [`State`] is an"] # [doc = " element of `{unreachable} ∪ (P -> V)`. This again forms a lattice, where the bottom element is"] # [doc = " `unreachable` and the top element is the mapping `p ↦ ⊤`. Note that the mapping `p ↦ ⊥` is not"] # [doc = " the bottom element (because joining an unreachable and any other reachable state yields a"] # [doc = " reachable state). All operations on unreachable states are ignored."] # [doc = ""] # [doc = " Flooding means assigning a value (by default `⊤`) to all tracked projections of a given place."] # [derive (PartialEq , Eq , Debug)] pub enum State < V > { Unreachable , Reachable (StateData < V >) , }
};
}
