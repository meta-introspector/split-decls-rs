// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CrateInfo",
decl_type: "function",
source_file: "./src/crate_finder.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Serialize", "Clone", "String", "Deserialize", "CrateInfo", "PathBuf"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! CrateInfo {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct CrateInfo { pub name : String , pub original_path : PathBuf , }
    };
}

CrateInfo!();