// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "TypeManifold",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: [],
uses: ["HashMap", "String", "Serialize", "Debug", "Default", "TypeManifold", "Deserialize", "Vec"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! TypeManifold {
    () => {
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct TypeManifold { pub dimensions : [HashMap < String , f64 > ; 8] , pub relationships : HashMap < String , Vec < String > > , pub embeddings : HashMap < String , [f64 ; 8] > , }
    };
}

TypeManifold!();