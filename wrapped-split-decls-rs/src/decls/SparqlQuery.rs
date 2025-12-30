// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SparqlQuery",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: ["QueryType"],
uses: ["Serialize", "SparqlQuery", "Option", "Debug", "String", "Deserialize", "QueryType"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        QueryType!();
    };
}

macro_rules! SparqlQuery {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct SparqlQuery { pub name : String , pub query_type : QueryType , pub threshold : Option < f64 > , pub limit : Option < usize > , pub target_template : String , }
    };
}

SparqlQuery!();