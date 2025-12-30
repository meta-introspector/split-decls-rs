// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PatternStats",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: [],
uses: ["Vec", "Debug", "String", "Deserialize", "Default", "Serialize", "PatternStats"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! PatternStats {
    () => {
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct PatternStats { pub count : u64 , pub ast_nodes : Vec < String > , pub complexity_score : f64 , }
    };
}

PatternStats!();