// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ComplexityResult",
decl_type: "function",
source_file: "./src/sparql_probe_bridge.rs",
source_crate: ".",
deps: [],
uses: ["ComplexityResult", "Debug", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ComplexityResult {
    () => {
        # [derive (Debug)] pub struct ComplexityResult { pub function_name : String , pub complexity : f64 , pub file_path : String , pub frequency : usize , }
    };
}

ComplexityResult!();