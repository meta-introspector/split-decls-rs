// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "sanitize_lean_id",
decl_type: "function",
source_file: "./src/bootstrap_tracer.rs",
source_crate: ".",
deps: [],
uses: ["String"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! sanitize_lean_id {
    () => {
        fn sanitize_lean_id (id : & str) -> String { id . chars () . map (| c | if c . is_alphanumeric () || c == '_' { c } else { '_' }) . collect :: < String > () . trim_start_matches (| c : char | c . is_numeric ()) . to_string () }
    };
}

sanitize_lean_id!();