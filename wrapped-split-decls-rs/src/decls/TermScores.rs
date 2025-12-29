// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TermScores",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/terms.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Clone", "Debug", "PartialEq", "Deserialize", "TermScores"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! TermScores {
    () => {
        # [derive (Debug , Clone , PartialEq , Serialize , Deserialize)] pub struct TermScores { pub local_score : f64 , pub module_score : f64 , pub global_score : f64 , }
    };
}

TermScores!();