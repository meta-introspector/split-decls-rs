// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "CompilerEigenform",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: ["PrincipalComponent"],
uses: ["Debug", "Compressed", "Vec", "PrincipalComponent", "Principal", "Reconstruction", "CompilerEigenform"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        PrincipalComponent!();
    };
}

macro_rules! CompilerEigenform {
    () => {
        deps!();
        # [derive (Debug)] pub struct CompilerEigenform { # [doc = " Principal components of rustc"] pub components : Vec < PrincipalComponent > , # [doc = " Compressed compiler representation"] pub compressed_rustc : Vec < f64 > , # [doc = " Reconstruction capability"] pub fidelity : f64 , }
    };
}

CompilerEigenform!();