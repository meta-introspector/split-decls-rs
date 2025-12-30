// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstReflector",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["AstProbe"],
uses: ["Default", "Vec", "Debug", "HashMap", "PathBuf", "AstReflector", "String", "AstProbe"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstProbe!();
    };
}

macro_rules! AstReflector {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct AstReflector { probes : Vec < AstProbe > , collected_data : HashMap < String , Vec < String > > , transformation_count : HashMap < String , usize > , current_file : PathBuf , }
    };
}

AstReflector!();