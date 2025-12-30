// Generated macro for QueryType (enum)
macro_rules! Depcrate_sparql_probe_bridgeQueryType {
() => {
// Module: crate::sparql_probe_bridge
// Provides: {"QueryType"}
// Dependencies: {}
# [derive (Debug , Serialize , Deserialize)] pub enum QueryType { MaxComplexity , ComplexityAbove (f64) , FrequencyAbove (usize) , PatternMatch (String) , CrossLayerRelation , }
};
}
