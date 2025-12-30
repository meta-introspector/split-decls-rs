// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CargoGuidedAnalysis",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: ["CargoBuildOrder", "CargoLockPreservation"],
uses: ["CargoGuidedAnalysis", "CargoBuildOrder", "CargoLockPreservation", "String", "Vec", "PathBuf", "HashMap"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        CargoBuildOrder!();
        CargoLockPreservation!();
    };
}

macro_rules! CargoGuidedAnalysis {
    () => {
        deps!();
        pub struct CargoGuidedAnalysis { pub root_path : PathBuf , pub cargo_lock : CargoLockPreservation , pub build_orders : HashMap < String , CargoBuildOrder > , pub source_analysis_queue : Vec < PathBuf > , }
    };
}

CargoGuidedAnalysis!();