// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ModuleNotFoundReport",
decl_type: "function",
source_file: "./src/eager_splitter.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "PathBuf", "String", "ModuleNotFoundReport", "Clone"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ModuleNotFoundReport {
    () => {
        # [derive (Debug , Clone)] pub struct ModuleNotFoundReport { pub crate_name : String , pub module_name : String , pub error_message : String , pub generated_file_path : PathBuf , }
    };
}

ModuleNotFoundReport!();