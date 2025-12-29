// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SparqlQuery",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: ["QueryType"],
uses: ["SparqlQuery", "Serialize", "QueryType", "String", "Deserialize", "Debug", "Option"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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