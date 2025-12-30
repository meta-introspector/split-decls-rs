// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "scan_rust_files_recursive",
decl_type: "function",
source_file: "./src/all_file_scanner.rs",
source_crate: ".",
deps: [],
uses: ["Fn", "PathBuf", "Ok", "Vec", "Result", "Item", "Path"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! scan_rust_files_recursive {
    () => {
        fn scan_rust_files_recursive (dir : & Path , files : & mut Vec < (PathBuf , Vec < Item >) >) -> Result < () > { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_dir () { scan_rust_files_recursive (& path , files) ? ; } else if path . extension () . map_or (false , | ext | ext == "rs") { if let Ok (content) = fs :: read_to_string (& path) { if let Ok (parsed) = syn :: parse_file (& content) { let functions : Vec < Item > = parsed . items . into_iter () . filter (| item | matches ! (item , Item :: Fn (_))) . collect () ; if ! functions . is_empty () { files . push ((path , functions)) ; } } } } } Ok (()) }
    };
}

scan_rust_files_recursive!();