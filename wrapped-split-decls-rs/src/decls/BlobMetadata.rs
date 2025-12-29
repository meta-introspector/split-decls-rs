// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "BlobMetadata",
decl_type: "function",
source_file: "./src/rdf_url_blob.rs",
source_crate: ".",
deps: [],
uses: ["Debug", "String", "BlobMetadata", "Vec", "Clone"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! BlobMetadata {
    () => {
        # [derive (Debug , Clone)] pub struct BlobMetadata { pub timestamp : String , pub system_version : String , pub macro_count : usize , pub export_capabilities : Vec < String > , }
    };
}

BlobMetadata!();