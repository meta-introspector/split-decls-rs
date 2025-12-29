// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "load_config",
decl_type: "function",
source_file: "./src/patch_config.rs",
source_crate: ".",
deps: [],
uses: ["Ok", "Result", "SplitDeclsConfig"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! load_config {
    () => {
        pub fn load_config () -> Result < SplitDeclsConfig > { let config_path = "split-decls-rs.toml" ; let config_content = std :: fs :: read_to_string (config_path) ? ; let config : SplitDeclsConfig = toml :: from_str (& config_content) ? ; Ok (config) }
    };
}

load_config!();