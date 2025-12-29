// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstStatistics",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: ["ParameterStats", "ConstructorStats", "PatternStats", "TypeManifold", "VariantStats"],
uses: ["Default", "String", "Deserialize", "ParameterStats", "ConstructorStats", "PatternStats", "TypeManifold", "Serialize", "VariantStats", "AstStatistics", "HashMap", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ParameterStats!();
        ConstructorStats!();
        PatternStats!();
        TypeManifold!();
        VariantStats!();
    };
}

macro_rules! AstStatistics {
    () => {
        deps!();
        # [derive (Debug , Default , Serialize , Deserialize)] pub struct AstStatistics { pub enum_variants : HashMap < String , VariantStats > , pub constructors : HashMap < String , ConstructorStats > , pub parameters : HashMap < String , ParameterStats > , pub usage_patterns : HashMap < String , PatternStats > , pub type_manifold : TypeManifold , }
    };
}

AstStatistics!();