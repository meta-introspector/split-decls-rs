// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BootstrapCache",
decl_type: "function",
source_file: "./src/bootstrap_cache.rs",
source_crate: ".",
deps: ["FileCache"],
uses: ["FileCache", "Debug", "Deserialize", "PathBuf", "BootstrapCache", "Default", "Serialize", "Option", "String", "HashMap"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        FileCache!();
    };
}

macro_rules! BootstrapCache {
    () => {
        deps!();
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct BootstrapCache { files : HashMap < PathBuf , FileCache > , git_tree_hash : Option < String > , }
    };
}

BootstrapCache!();