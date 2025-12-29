// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BootstrapCache",
decl_type: "function",
source_file: "./src/bootstrap_cache.rs",
source_crate: ".",
deps: ["FileCache"],
uses: ["Default", "HashMap", "FileCache", "String", "Option", "BootstrapCache", "Serialize", "PathBuf", "Debug", "Deserialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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