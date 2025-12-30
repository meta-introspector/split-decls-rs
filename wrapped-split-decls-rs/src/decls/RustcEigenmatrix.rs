// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RustcEigenmatrix",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: ["CompilerEigenform", "UrlMatrix", "RustcAnalysis"],
uses: ["Feature", "Eigenform", "Mathematical", "Option", "CompilerEigenform", "RustcEigenmatrix", "UrlMatrix", "Rustc", "Debug", "Eigenmatrix", "RustcAnalysis"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        CompilerEigenform!();
        UrlMatrix!();
        RustcAnalysis!();
    };
}

macro_rules! RustcEigenmatrix {
    () => {
        deps!();
        # [doc = " Rustc Eigenmatrix - Mathematical representation of the entire rustc compiler"] # [derive (Debug)] pub struct RustcEigenmatrix { # [doc = " Feature matrix of rustc components"] pub matrix : UrlMatrix , # [doc = " Rustc source analysis"] pub analysis : RustcAnalysis , # [doc = " Eigenform of the compiler"] pub eigenform : Option < CompilerEigenform > , }
    };
}

RustcEigenmatrix!();