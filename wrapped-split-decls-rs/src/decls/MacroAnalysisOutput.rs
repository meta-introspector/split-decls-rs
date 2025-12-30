// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "MacroAnalysisOutput",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/output_format.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Debug", "String", "MacroAnalysisOutput", "HashMap", "Deserialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! MacroAnalysisOutput {
    () => {
        # [derive (Serialize , Deserialize , Debug)] pub struct MacroAnalysisOutput { pub total_module_terms : usize , pub total_global_terms : usize , pub module_frequencies : HashMap < String , usize > , pub global_frequencies : HashMap < String , usize > , pub term_scores_by_macro : HashMap < String , HashMap < String , f64 > > , pub global_module_term_scores : HashMap < String , (f64 , f64) > , }
    };
}

MacroAnalysisOutput!();