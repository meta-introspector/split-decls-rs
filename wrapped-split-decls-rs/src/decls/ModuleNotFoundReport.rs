// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ModuleNotFoundReport",
decl_type: "function",
source_file: "./src/eager_splitter.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Clone", "String", "PathBuf", "ModuleNotFoundReport"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! ModuleNotFoundReport {
    () => {
        # [derive (Debug , Clone)] pub struct ModuleNotFoundReport { pub crate_name : String , pub module_name : String , pub error_message : String , pub generated_file_path : PathBuf , }
    };
}

ModuleNotFoundReport!();