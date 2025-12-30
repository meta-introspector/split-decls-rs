// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "create_example_sparql_config",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: ["SparqlToProbeConfig", "SparqlQuery", "AstNodeType", "QueryType", "ProbeTemplate"],
uses: ["Some", "SparqlToProbeConfig", "SparqlQuery", "HashMap", "None", "FrequencyAbove", "MaxComplexity", "AstNodeType", "QueryType", "ProbeTemplate", "Function", "ComplexityAbove"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SparqlToProbeConfig!();
        SparqlQuery!();
        AstNodeType!();
        QueryType!();
        ProbeTemplate!();
    };
}

macro_rules! create_example_sparql_config {
    () => {
        deps!();
        pub fn create_example_sparql_config () -> SparqlToProbeConfig { let mut probe_templates = HashMap :: new () ; probe_templates . insert ("complexity_wrapper" . to_string () , ProbeTemplate { node_type : AstNodeType :: Function , action_template : "wrap_complex_function" . to_string () , wrapper_function : Some ("complexity_ping" . to_string ()) , priority : 1 , }) ; probe_templates . insert ("frequency_monitor" . to_string () , ProbeTemplate { node_type : AstNodeType :: Function , action_template : "monitor_frequent_function" . to_string () , wrapper_function : Some ("frequency_tracker" . to_string ()) , priority : 2 , }) ; let queries = vec ! [SparqlQuery { name : "top_10_complex" . to_string () , query_type : QueryType :: MaxComplexity , threshold : None , limit : Some (10) , target_template : "complexity_wrapper" . to_string () , } , SparqlQuery { name : "high_complexity" . to_string () , query_type : QueryType :: ComplexityAbove (8.0) , threshold : Some (8.0) , limit : Some (20) , target_template : "complexity_wrapper" . to_string () , } , SparqlQuery { name : "frequent_functions" . to_string () , query_type : QueryType :: FrequencyAbove (100) , threshold : None , limit : Some (15) , target_template : "frequency_monitor" . to_string () , } ,] ; SparqlToProbeConfig { queries , probe_templates , } }
    };
}

create_example_sparql_config!();