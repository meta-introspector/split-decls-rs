// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "QueryType",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: [],
uses: ["FrequencyAbove", "Debug", "Serialize", "PatternMatch", "QueryType", "ComplexityAbove", "MaxComplexity", "Deserialize", "CrossLayerRelation", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! QueryType {
    () => {
        # [derive (Debug , Serialize , Deserialize)] pub enum QueryType { MaxComplexity , ComplexityAbove (f64) , FrequencyAbove (usize) , PatternMatch (String) , CrossLayerRelation , }
    };
}

QueryType!();