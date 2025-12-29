// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ComplexityMetrics",
decl_type: "function",
source_file: "./src/syn_mold.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "Debug", "Default", "ComplexityMetrics"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! ComplexityMetrics {
    () => {
        # [derive (Debug , Default , Clone)] pub struct ComplexityMetrics { pub parse_operations : u32 , pub visit_operations : u32 , pub transform_operations : u32 , pub generation_operations : u32 , pub total_complexity : f64 , pub max_depth : u32 , }
    };
}

ComplexityMetrics!();