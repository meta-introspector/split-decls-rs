// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CargoLockPreservation",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: ["PackageField", "PreservationTrace"],
uses: ["Vec", "HashMap", "PackageField", "PathBuf", "String", "Deserialize", "Serialize", "PreservationTrace", "CargoLockPreservation", "Clone", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        PackageField!();
        PreservationTrace!();
    };
}

macro_rules! CargoLockPreservation {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CargoLockPreservation { pub original_lock : PathBuf , pub split_decls_toml : PathBuf , pub bootstrap_stage : String , pub output2_stage : String , pub packages : Vec < PackageField > , pub preservation_map : HashMap < String , PreservationTrace > , }
    };
}

CargoLockPreservation!();