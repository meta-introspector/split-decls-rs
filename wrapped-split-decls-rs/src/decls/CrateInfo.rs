// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CrateInfo",
decl_type: "function",
source_file: "./src/crate_finder.rs",
source_crate: ".",
deps: [],
uses: ["PathBuf", "Clone", "Serialize", "Deserialize", "Debug", "String", "CrateInfo"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! CrateInfo {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct CrateInfo { pub name : String , pub original_path : PathBuf , }
    };
}

CrateInfo!();