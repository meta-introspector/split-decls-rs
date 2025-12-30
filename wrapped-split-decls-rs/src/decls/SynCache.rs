// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SynCache",
decl_type: "function",
source_file: "./src/syn_cache.rs",
source_crate: ".",
deps: ["FileCache"],
uses: ["HashMap", "String", "FileCache", "Debug", "Default", "SynCache"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        FileCache!();
    };
}

macro_rules! SynCache {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct SynCache { cache : HashMap < String , FileCache > , cache_file : String , }
    };
}

SynCache!();