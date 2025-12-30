// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TermScores",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/terms.rs",
source_crate: ".",
deps: [],
uses: ["Deserialize", "TermScores", "Debug", "PartialEq", "Serialize", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! TermScores {
    () => {
        # [derive (Debug , Clone , PartialEq , Serialize , Deserialize)] pub struct TermScores { pub local_score : f64 , pub module_score : f64 , pub global_score : f64 , }
    };
}

TermScores!();