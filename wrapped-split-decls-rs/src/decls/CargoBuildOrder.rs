// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CargoBuildOrder",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: ["BuildStep"],
uses: ["Deserialize", "Serialize", "PathBuf", "CargoBuildOrder", "Debug", "Vec", "BuildStep", "Clone", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        BuildStep!();
    };
}

macro_rules! CargoBuildOrder {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CargoBuildOrder { pub crate_name : String , pub crate_path : PathBuf , pub build_order : Vec < BuildStep > , pub dependencies_resolved : Vec < String > , pub dry_run_output : String , }
    };
}

CargoBuildOrder!();