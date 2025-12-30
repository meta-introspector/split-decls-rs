// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "FileCache",
decl_type: "function",
source_file: "./src/syn_cache.rs",
source_crate: ".",
deps: [],
uses: ["Deserialize", "Serialize", "Option", "Debug", "String", "Clone", "FileCache"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! FileCache {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct FileCache { pub content : String , pub modified_time : u64 , pub parsed_ast : Option < String > , }
    };
}

FileCache!();