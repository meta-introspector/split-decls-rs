// Generated macro for QueryRevisions (struct)
macro_rules! Depcrate_zalsa_localQueryRevisions {
() => {
// Module: crate::zalsa_local
// Provides: {"QueryRevisions"}
// Dependencies: {}
# [doc = " Summarizes \"all the inputs that a query used\" and \"all the outputs it has written to\"."] # [derive (Debug)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] pub (crate) struct QueryRevisions { # [doc = " The most revision in which some input changed."] pub (crate) changed_at : Revision , # [doc = " Minimum durability of the inputs to this query."] pub (crate) durability : Durability , # [doc = " How was this query computed?"] pub (crate) origin : QueryOrigin , # [doc = " [`InputAccumulatedValues::Empty`] if any input read during the query's execution"] # [doc = " has any direct or indirect accumulated values."] # [doc = ""] # [doc = " Note that this field could be in `QueryRevisionsExtra` as it is only relevant"] # [doc = " for accumulators, but we get it for free anyways due to padding."] # [cfg (feature = "accumulator")] # [cfg_attr (feature = "persistence" , serde (skip))] pub (super) accumulated_inputs : AtomicInputAccumulatedValues , # [doc = " Are the `cycle_heads` verified to not be provisional anymore?"] # [doc = ""] # [doc = " Note that this field could be in `QueryRevisionsExtra` as it is only"] # [doc = " relevant for queries that participate in a cycle, but we get it for"] # [doc = " free anyways due to padding."] # [cfg_attr (feature = "persistence" , serde (with = "persistence::verified_final"))] pub (super) verified_final : AtomicBool , # [doc = " Lazily allocated state."] pub (super) extra : QueryRevisionsExtra , }
};
}
