// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "scan_all_rust_files",
decl_type: "function",
source_file: "./src/all_file_scanner.rs",
source_crate: ".",
deps: [],
uses: ["Rust", "Item", "Ok", "Result", "Path", "PathBuf", "Vec", "Scan"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! scan_all_rust_files {
    () => {
        # [doc = " Scan all Rust source files in a crate and extract functions"] pub fn scan_all_rust_files (crate_path : & Path) -> Result < Vec < (PathBuf , Vec < Item >) > > { let mut all_files = Vec :: new () ; let src_dir = crate_path . join ("src") ; if src_dir . exists () { scan_rust_files_recursive (& src_dir , & mut all_files) ? ; } Ok (all_files) }
    };
}

scan_all_rust_files!();