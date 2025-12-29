// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "find_cargo_tomls",
decl_type: "function",
source_file: "./src/auto_workspace_generator.rs",
source_crate: ".",
deps: [],
uses: ["Cargo.toml", "Result", "PathBuf", "Some", "Ok", "Vec", "Path"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! find_cargo_tomls {
    () => {
        fn find_cargo_tomls (dir : & Path) -> Result < Vec < PathBuf > > { let mut cargo_tomls = Vec :: new () ; if dir . is_dir () { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . file_name () == Some ("Cargo.toml" . as_ref ()) { cargo_tomls . push (path) ; } else if path . is_dir () && ! should_skip_dir (& path) { cargo_tomls . extend (find_cargo_tomls (& path) ?) ; } } } Ok (cargo_tomls) }
    };
}

find_cargo_tomls!();