// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RdfUrlBlob",
decl_type: "function",
source_file: "./src/rdf_url_blob.rs",
source_crate: ".",
deps: ["BlobMetadata"],
uses: ["RDF", "Clone", "URL", "Complete", "RDF/Turtle", "BlobMetadata", "Metadata", "Blob", "Debug", "Base64", "RdfUrlBlob", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        BlobMetadata!();
    };
}

macro_rules! RdfUrlBlob {
    () => {
        deps!();
        # [doc = " RDF URL Blob - Complete system state as a URL that can be piped as stdin"] # [derive (Debug , Clone)] pub struct RdfUrlBlob { # [doc = " RDF/Turtle representation of system state"] pub rdf_content : String , # [doc = " Base64 encoded blob for URL transport"] pub url_blob : String , # [doc = " Metadata about the state"] pub metadata : BlobMetadata , }
    };
}

RdfUrlBlob!();