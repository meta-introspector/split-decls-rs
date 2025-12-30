// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstStatistics",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: ["ConstructorStats", "TypeManifold", "VariantStats", "PatternStats", "ParameterStats"],
uses: ["ConstructorStats", "Debug", "Serialize", "Deserialize", "TypeManifold", "AstStatistics", "Default", "VariantStats", "HashMap", "PatternStats", "String", "ParameterStats"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ConstructorStats!();
        TypeManifold!();
        VariantStats!();
        PatternStats!();
        ParameterStats!();
    };
}

macro_rules! AstStatistics {
    () => {
        deps!();
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct AstStatistics { pub enum_variants : HashMap < String , VariantStats > , pub constructors : HashMap < String , ConstructorStats > , pub parameters : HashMap < String , ParameterStats > , pub usage_patterns : HashMap < String , PatternStats > , pub type_manifold : TypeManifold , }
    };
}

AstStatistics!();