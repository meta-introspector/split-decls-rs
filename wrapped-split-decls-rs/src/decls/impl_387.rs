// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_387",
decl_type: "function",
source_file: "./src/source_tracker.rs",
source_crate: ".",
deps: ["SourceLocation", "SourceMap"],
uses: ["Option", "HashMap", "SourceLocation", "SourceMap"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SourceLocation!();
        SourceMap!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl SourceMap { pub fn new () -> Self { Self { mappings : HashMap :: new () , } } pub fn add_mapping (& mut self , offset : usize , location : SourceLocation) { self . mappings . insert (offset , location) ; } pub fn get_location (& self , offset : usize) -> Option < & SourceLocation > { self . mappings . get (& offset) } }
    };
}

impl_387!();