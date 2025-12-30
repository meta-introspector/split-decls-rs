// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BuildStep",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "Serialize", "Deserialize", "String", "Option", "Debug", "BuildStep"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! BuildStep {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BuildStep { pub step_number : usize , pub action : String , pub target : String , pub duration_estimate : Option < f64 > , }
    };
}

BuildStep!();