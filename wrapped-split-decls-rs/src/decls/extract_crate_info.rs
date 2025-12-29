// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "extract_crate_info",
decl_type: "function",
source_file: "./src/auto_workspace_generator.rs",
source_crate: ".",
deps: ["CrateInfo"],
uses: ["None", "Some", "Value", "Path", "Result", "Ok", "Option", "CrateInfo"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        CrateInfo!();
    };
}

macro_rules! extract_crate_info {
    () => {
        deps!();
        fn extract_crate_info (cargo_path : & Path) -> Result < Option < CrateInfo > > { let content = fs :: read_to_string (cargo_path) ? ; let toml : Value = toml :: from_str (& content) ? ; if let Some (package) = toml . get ("package") { if let Some (name) = package . get ("name") . and_then (| n | n . as_str ()) { return Ok (Some (CrateInfo { name : name . to_string () , })) ; } } Ok (None) }
    };
}

extract_crate_info!();