// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ConstructorStats",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: [],
uses: ["Deserialize", "String", "ConstructorStats", "Vec", "Serialize", "Default", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! ConstructorStats {
    () => {
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct ConstructorStats { pub count : u64 , pub parameter_types : Vec < String > , pub return_type : String , pub usage_contexts : Vec < String > , }
    };
}

ConstructorStats!();