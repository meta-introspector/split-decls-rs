// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "ExtractedDeclMetadata",
decl_type: "function",
source_file: "./src/extracted_decl.rs",
source_crate: ".",
deps: [],
uses: ["Serialize", "Debug", "Default", "ExtractedDeclMetadata", "Option", "Deserialize", "Hash", "Eq", "Clone", "String", "Vec", "PartialEq"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! ExtractedDeclMetadata {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize , Default)] pub struct ExtractedDeclMetadata { pub ast_depth : usize , pub ast_node_count : usize , pub inputs_hash : Option < String > , pub output_hash : Option < String > , pub rings_of_sizes : Vec < usize > , pub zkp_witness_hash : Option < String > , }
    };
}

ExtractedDeclMetadata!();