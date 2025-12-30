// Generated macro for macro_218 (macro)
macro_rules! Depcrate_json_schema_impls_indexmap2macro_218 {
() => {
// Module: crate::json_schema_impls::indexmap2
// Provides: {"macro_218"}
// Dependencies: {}
forward_impl ! ((< K : JsonSchema , V : JsonSchema , H > JsonSchema for IndexMap < K , V , H >) => BTreeMap < K , V >) ;
};
}
