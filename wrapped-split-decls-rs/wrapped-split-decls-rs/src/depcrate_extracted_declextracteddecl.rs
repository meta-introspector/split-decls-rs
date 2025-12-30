// Generated macro for ExtractedDecl (struct)
macro_rules! Depcrate_extracted_declExtractedDecl {
() => {
// Module: crate::extracted_decl
// Provides: {"ExtractedDecl"}
// Dependencies: {}
# [doc = " Represents a single extracted declaration."] # [derive (Debug , Clone)] pub struct ExtractedDecl { pub name : String , pub kind : String , pub content : TokenStream , pub metadata : ExtractedDeclMetadata , pub source_map : std :: collections :: HashMap < usize , SourceLocation > , }
};
}
