// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CargoLockPreservation",
decl_type: "function",
source_file: "./src/cargo_guided_analysis.rs",
source_crate: ".",
deps: ["PreservationTrace", "PackageField"],
uses: ["Serialize", "PathBuf", "CargoLockPreservation", "PreservationTrace", "Vec", "HashMap", "Deserialize", "Debug", "String", "PackageField", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        PreservationTrace!();
        PackageField!();
    };
}

macro_rules! CargoLockPreservation {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CargoLockPreservation { pub original_lock : PathBuf , pub split_decls_toml : PathBuf , pub bootstrap_stage : String , pub output2_stage : String , pub packages : Vec < PackageField > , pub preservation_map : HashMap < String , PreservationTrace > , }
    };
}

CargoLockPreservation!();