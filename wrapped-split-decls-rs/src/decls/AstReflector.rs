// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstReflector",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["AstProbe"],
uses: ["Default", "Debug", "AstProbe", "PathBuf", "HashMap", "AstReflector", "String", "Vec"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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