// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SimpleCrateInfo",
decl_type: "function",
source_file: "./src/simple_crate_info.rs",
source_crate: ".",
deps: [],
uses: ["PartialEq", "Deserialize", "String", "Clone", "Debug", "PathBuf", "Serialize", "SimpleCrateInfo", "Eq"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! SimpleCrateInfo {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct SimpleCrateInfo { pub name : String , pub version : String , pub manifest_path : PathBuf , }
    };
}

SimpleCrateInfo!();