// Generated macro for QueryEdge (struct)
macro_rules! Depcrate_zalsa_localQueryEdge {
() => {
// Module: crate::zalsa_local
// Provides: {"QueryEdge"}
// Dependencies: {}
# [doc = " An input or output query edge."] # [doc = ""] # [doc = " This type is a packed version of `QueryEdgeKind`, tagging the `IngredientIndex`"] # [doc = " in `key` with a discriminator for the input and output variants without increasing"] # [doc = " the size of the type. Notably, this type is 12 bytes as opposed to the 16 byte"] # [doc = " `QueryEdgeKind`, which is meaningful as inputs and outputs are stored contiguously."] # [derive (Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "persistence" , serde (transparent))] pub struct QueryEdge { key : DatabaseKeyIndex , }
};
}
