// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "should_skip_dir",
decl_type: "function",
source_file: "./src/auto_workspace_generator.rs",
source_crate: ".",
deps: [],
uses: ["Path"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! should_skip_dir {
    () => {
        fn should_skip_dir (path : & Path) -> bool { let name = path . file_name () . unwrap () . to_string_lossy () ; matches ! (name . as_ref () , "target" | ".git" | "node_modules") }
    };
}

should_skip_dir!();