// Generated macro for QueryRevisionsExtra (struct)
macro_rules! Depcrate_zalsa_localQueryRevisionsExtra {
() => {
// Module: crate::zalsa_local
// Provides: {"QueryRevisionsExtra"}
// Dependencies: {}
# [doc = " Data on `QueryRevisions` that is lazily allocated to save memory"] # [doc = " in the common case."] # [doc = ""] # [doc = " In particular, not all queries create tracked structs, participate"] # [doc = " in cycles, or create accumulators."] # [derive (Debug , Default)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "persistence" , serde (transparent))] pub (crate) struct QueryRevisionsExtra (Option < Box < QueryRevisionsExtraInner > >) ;
};
}
