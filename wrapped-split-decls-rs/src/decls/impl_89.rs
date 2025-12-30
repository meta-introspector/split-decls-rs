// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_89",
decl_type: "function",
source_file: "./src/patch_config.rs",
source_crate: ".",
deps: ["PatchConfig"],
uses: ["Default", "PatchConfig", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        PatchConfig!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl Default for PatchConfig { fn default () -> Self { Self { generated_workspace_member : Vec :: new () , generated_workspace_dependency : Vec :: new () , generated_crate_dependency : Vec :: new () , } } }
    };
}

impl_89!();