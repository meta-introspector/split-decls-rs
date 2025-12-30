// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "find_all_rust_files",
decl_type: "function",
source_file: "./src/eager_splitter.rs",
source_crate: ".",
deps: [],
uses: ["PathBuf", "Ok", "Path", "Result", "Recursively", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! find_all_rust_files {
    () => {
        # [doc = " Recursively finds all .rs files in src directory that cargo would build"] fn find_all_rust_files (src_dir : & std :: path :: Path) -> Result < Vec < std :: path :: PathBuf > > { let mut rust_files = Vec :: new () ; if ! src_dir . exists () { return Ok (rust_files) ; } for entry in std :: fs :: read_dir (src_dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { rust_files . push (path) ; } else if path . is_dir () { rust_files . extend (find_all_rust_files (& path) ?) ; } } Ok (rust_files) }
    };
}

find_all_rust_files!();