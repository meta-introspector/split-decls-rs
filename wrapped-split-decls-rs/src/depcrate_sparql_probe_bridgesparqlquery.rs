// Generated macro for SparqlQuery (struct)
macro_rules! Depcrate_sparql_probe_bridgeSparqlQuery {
() => {
// Module: crate::sparql_probe_bridge
// Provides: {"SparqlQuery"}
// Dependencies: {}
# [derive (Debug , Serialize , Deserialize)] pub struct SparqlQuery { pub name : String , pub query_type : QueryType , pub threshold : Option < f64 > , pub limit : Option < usize > , pub target_template : String , }
};
}
