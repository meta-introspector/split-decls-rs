// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PatternType",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: [],
uses: ["AttributeProcessing", "MacroExpansion", "Debug", "Clone", "PatternType", "VisitMut", "ParseQuote", "TokenStreamGeneration"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! PatternType {
    () => {
        # [derive (Debug , Clone)] pub enum PatternType { ParseQuote , VisitMut , TokenStreamGeneration , AttributeProcessing , MacroExpansion , }
    };
}

PatternType!();