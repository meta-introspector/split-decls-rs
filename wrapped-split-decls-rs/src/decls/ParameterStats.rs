// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ParameterStats",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: [],
uses: ["Deserialize", "Vec", "Debug", "Default", "ParameterStats", "String", "Serialize"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ParameterStats {
    () => {
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct ParameterStats { pub count : u64 , pub type_name : String , pub positions : Vec < usize > , pub associated_functions : Vec < String > , }
    };
}

ParameterStats!();