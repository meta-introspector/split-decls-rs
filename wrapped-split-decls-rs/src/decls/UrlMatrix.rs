// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "UrlMatrix",
decl_type: "function",
source_file: "./src/url_matrix.rs",
source_crate: ".",
deps: ["EigenForm"],
uses: ["Collection", "RDF", "Clone", "Vec", "UrlMatrix", "String", "Feature", "Option", "Debug", "Matrix", "URL", "Eigenform", "EigenForm"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        EigenForm!();
    };
}

macro_rules! UrlMatrix {
    () => {
        deps!();
        # [doc = " URL Matrix - Collection of RDF URL blobs as a compressible matrix"] # [derive (Debug , Clone)] pub struct UrlMatrix { # [doc = " Matrix of URL blobs (rows = states, cols = features)"] pub matrix : Vec < Vec < f64 > > , # [doc = " URL blob sources"] pub urls : Vec < String > , # [doc = " Feature names (macro names, types, etc.)"] pub features : Vec < String > , # [doc = " Eigenform compression"] pub eigenform : Option < EigenForm > , }
    };
}

UrlMatrix!();