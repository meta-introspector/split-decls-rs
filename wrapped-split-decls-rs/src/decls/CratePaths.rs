// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CratePaths",
decl_type: "function",
source_file: "./src/paths.rs",
source_crate: ".",
deps: [],
uses: ["CratePaths", "PathBuf", "Encapsulates", "String", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! CratePaths {
    () => {
        # [doc = " Encapsulates all relevant file paths for a target crate."] pub struct CratePaths { pub crate_path : PathBuf , pub crate_name : String , pub source_files : Vec < PathBuf > , pub build_rs_path : PathBuf , pub cargo_toml_path : PathBuf , pub decls_output_dir : PathBuf , pub target_config_path : PathBuf , pub output_crate_path : PathBuf , }
    };
}

CratePaths!();