// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PatternType",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "MacroExpansion", "Debug", "TokenStreamGeneration", "ParseQuote", "AttributeProcessing", "PatternType", "VisitMut"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! PatternType {
    () => {
        # [derive (Debug , Clone)] pub enum PatternType { ParseQuote , VisitMut , TokenStreamGeneration , AttributeProcessing , MacroExpansion , }
    };
}

PatternType!();