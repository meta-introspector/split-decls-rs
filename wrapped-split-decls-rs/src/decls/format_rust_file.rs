// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "format_rust_file",
decl_type: "function",
source_file: "./src/rustfmt_utils.rs",
source_crate: ".",
deps: [],
uses: ["DEBUG", "Skipping", "Ok", "Path", "Result", "SPLIT_DECLS_DEBUG", "String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! format_rust_file {
    () => {
        pub fn format_rust_file (content : & str , path : & Path) -> Result < String > { if std :: env :: var ("SPLIT_DECLS_DEBUG") . is_ok () { eprintln ! ("DEBUG: Skipping formatting for {}" , path . display ()) ; } Ok (content . to_string ()) }
    };
}

format_rust_file!();