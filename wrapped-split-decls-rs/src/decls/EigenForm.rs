// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "EigenForm",
decl_type: "function",
source_file: "./src/url_matrix.rs",
source_crate: ".",
deps: [],
uses: ["Compression", "Eigenvectors", "Clone", "EigenForm", "Vec", "Compressed", "Debug", "Eigenvalues"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! EigenForm {
    () => {
        # [derive (Debug , Clone)] pub struct EigenForm { # [doc = " Eigenvalues (principal components)"] pub eigenvalues : Vec < f64 > , # [doc = " Eigenvectors (basis vectors)"] pub eigenvectors : Vec < Vec < f64 > > , # [doc = " Compressed representation"] pub compressed : Vec < f64 > , # [doc = " Compression ratio"] pub ratio : f64 , }
    };
}

EigenForm!();