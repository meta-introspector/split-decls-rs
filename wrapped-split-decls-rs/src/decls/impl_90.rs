// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_90",
decl_type: "function",
source_file: "./src/patch_config.rs",
source_crate: ".",
deps: ["PatchConfig"],
uses: ["Patch", "PatchConfig", "Path", "Result", "Ok"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        PatchConfig!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl PatchConfig { pub fn load_from_file (path : & std :: path :: Path) -> anyhow :: Result < Self > { if ! path . exists () { anyhow :: bail ! ("Patch config file not found at {}" , path . display ()) ; } let content = std :: fs :: read_to_string (path) ? ; let config : Self = toml :: from_str (& content) ? ; Ok (config) } }
    };
}

impl_90!();