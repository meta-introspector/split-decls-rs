// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "format_rust_file",
decl_type: "function",
source_file: "./src/rustfmt_utils.rs",
source_crate: ".",
deps: [],
uses: ["Skipping", "Path", "Ok", "Result", "String", "SPLIT_DECLS_DEBUG", "DEBUG"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! format_rust_file {
    () => {
        pub fn format_rust_file (content : & str , path : & Path) -> Result < String > { if std :: env :: var ("SPLIT_DECLS_DEBUG") . is_ok () { eprintln ! ("DEBUG: Skipping formatting for {}" , path . display ()) ; } Ok (content . to_string ()) }
    };
}

format_rust_file!();