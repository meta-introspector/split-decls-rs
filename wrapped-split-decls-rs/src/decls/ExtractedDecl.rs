// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ExtractedDecl",
decl_type: "function",
source_file: "./src/extracted_decl.rs",
source_crate: ".",
deps: ["ExtractedDeclMetadata", "SourceLocation"],
uses: ["Clone", "TokenStream", "HashMap", "ExtractedDeclMetadata", "SourceLocation", "Debug", "String", "Represents", "ExtractedDecl"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        ExtractedDeclMetadata!();
        SourceLocation!();
    };
}

macro_rules! ExtractedDecl {
    () => {
        deps!();
        # [doc = " Represents a single extracted declaration."] # [derive (Debug , Clone)] pub struct ExtractedDecl { pub name : String , pub kind : String , pub content : TokenStream , pub metadata : ExtractedDeclMetadata , pub source_map : std :: collections :: HashMap < usize , SourceLocation > , }
    };
}

ExtractedDecl!();