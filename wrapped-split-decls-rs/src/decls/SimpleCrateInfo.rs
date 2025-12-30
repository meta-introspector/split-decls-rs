// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SimpleCrateInfo",
decl_type: "function",
source_file: "./src/simple_crate_info.rs",
source_crate: ".",
deps: [],
uses: ["Eq", "SimpleCrateInfo", "Deserialize", "Debug", "Clone", "PathBuf", "String", "Serialize", "PartialEq"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SimpleCrateInfo {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Serialize , Deserialize)] pub struct SimpleCrateInfo { pub name : String , pub version : String , pub manifest_path : PathBuf , }
    };
}

SimpleCrateInfo!();