// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "VariantStats",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Deserialize", "Vec", "String", "Default", "Serialize", "VariantStats"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! VariantStats {
    () => {
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct VariantStats { pub count : u64 , pub contexts : Vec < String > , pub dependencies : Vec < String > , pub patterns : Vec < String > , }
    };
}

VariantStats!();