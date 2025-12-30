// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TermAnalysis",
decl_type: "function",
source_file: "./src/macro_analyzer_parts/analysis_data.rs",
source_crate: ".",
deps: ["Term"],
uses: ["String", "TermAnalysis", "Vec", "HashMap", "Debug", "Default", "Term"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        Term!();
    };
}

macro_rules! TermAnalysis {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct TermAnalysis { pub terms_per_macro : HashMap < String , Vec < Term > > , pub module_frequencies : HashMap < Term , usize > , pub global_frequencies : HashMap < Term , usize > , pub total_module_terms : usize , pub total_global_terms : usize , pub term_detailed_scores : HashMap < Term , (f64 , f64 , HashMap < String , f64 >) > , }
    };
}

TermAnalysis!();