// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "QueryType",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: [],
uses: ["String", "Debug", "Serialize", "QueryType", "PatternMatch", "MaxComplexity", "Deserialize", "FrequencyAbove", "ComplexityAbove", "CrossLayerRelation"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! QueryType {
    () => {
        # [derive (Debug , Serialize , Deserialize)] pub enum QueryType { MaxComplexity , ComplexityAbove (f64) , FrequencyAbove (usize) , PatternMatch (String) , CrossLayerRelation , }
    };
}

QueryType!();