// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RustcAnalysis",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: ["CrateFeatures", "CompilerPhase", "ComplexityMetrics"],
uses: ["Debug", "HashMap", "CrateFeatures", "String", "Vec", "Complexity", "CompilerPhase", "All", "Compiler", "RustcAnalysis", "Total", "ComplexityMetrics"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        CrateFeatures!();
        CompilerPhase!();
        ComplexityMetrics!();
    };
}

macro_rules! RustcAnalysis {
    () => {
        deps!();
        # [derive (Debug)] pub struct RustcAnalysis { # [doc = " All rustc crates and their relationships"] pub crates : HashMap < String , CrateFeatures > , # [doc = " Compiler phases (parsing, analysis, codegen, etc.)"] pub phases : Vec < CompilerPhase > , # [doc = " Total lines of code"] pub total_loc : usize , # [doc = " Complexity metrics"] pub complexity : ComplexityMetrics , }
    };
}

RustcAnalysis!();