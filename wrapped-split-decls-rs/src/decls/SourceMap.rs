// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SourceMap",
decl_type: "function",
source_file: "./src/source_tracker.rs",
source_crate: ".",
deps: ["SourceLocation"],
uses: ["HashMap", "SourceLocation", "SourceMap", "Clone", "Debug", "Struct"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SourceLocation!();
    };
}

macro_rules! SourceMap {
    () => {
        deps!();
        # [doc = " Struct to store source mapping for a token stream"] # [derive (Debug , Clone)] pub struct SourceMap { pub mappings : HashMap < usize , SourceLocation > , }
    };
}

SourceMap!();