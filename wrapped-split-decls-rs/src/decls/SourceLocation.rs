// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SourceLocation",
decl_type: "function",
source_file: "./src/extracted_decl.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "Eq", "String", "Clone", "Serialize", "SourceLocation", "Struct", "Hash", "Deserialize", "PartialEq"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! SourceLocation {
    () => {
        # [doc = " Struct to store source location information"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub struct SourceLocation { pub file : String , pub line : usize , pub column : usize , }
    };
}

SourceLocation!();