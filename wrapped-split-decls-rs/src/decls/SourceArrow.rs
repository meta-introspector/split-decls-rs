// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SourceArrow",
decl_type: "function",
source_file: "./src/conformal_field_theory.rs",
source_crate: ".",
deps: [],
uses: ["Clone", "Serialize", "Deserialize", "Debug", "SourceArrow", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SourceArrow {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SourceArrow { pub file_path : String , pub line : usize , pub column : usize , pub angle : f64 , pub length : f64 , }
    };
}

SourceArrow!();