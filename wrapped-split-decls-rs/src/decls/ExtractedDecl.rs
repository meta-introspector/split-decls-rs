// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ExtractedDecl",
decl_type: "function",
source_file: "./src/extracted_decl.rs",
source_crate: ".",
deps: ["SourceLocation", "ExtractedDeclMetadata"],
uses: ["Represents", "String", "Clone", "HashMap", "SourceLocation", "ExtractedDecl", "TokenStream", "Debug", "ExtractedDeclMetadata"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SourceLocation!();
        ExtractedDeclMetadata!();
    };
}

macro_rules! ExtractedDecl {
    () => {
        deps!();
        # [doc = " Represents a single extracted declaration."] # [derive (Debug , Clone)] pub struct ExtractedDecl { pub name : String , pub kind : String , pub content : TokenStream , pub metadata : ExtractedDeclMetadata , pub source_map : std :: collections :: HashMap < usize , SourceLocation > , }
    };
}

ExtractedDecl!();