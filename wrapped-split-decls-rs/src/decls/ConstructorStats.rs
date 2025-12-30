// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ConstructorStats",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Vec", "Default", "ConstructorStats", "String", "Deserialize", "Serialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ConstructorStats {
    () => {
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct ConstructorStats { pub count : u64 , pub parameter_types : Vec < String > , pub return_type : String , pub usage_contexts : Vec < String > , }
    };
}

ConstructorStats!();