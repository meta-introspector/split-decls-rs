// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CompilerPhase",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: [],
uses: ["CompilerPhase", "Vec", "Debug", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! CompilerPhase {
    () => {
        # [derive (Debug)] pub struct CompilerPhase { pub name : String , pub crates : Vec < String > , pub complexity : f64 , }
    };
}

CompilerPhase!();