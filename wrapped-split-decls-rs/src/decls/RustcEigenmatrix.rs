// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "RustcEigenmatrix",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: ["UrlMatrix", "RustcAnalysis", "CompilerEigenform"],
uses: ["Feature", "UrlMatrix", "RustcEigenmatrix", "RustcAnalysis", "Mathematical", "CompilerEigenform", "Option", "Eigenform", "Rustc", "Eigenmatrix", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        UrlMatrix!();
        RustcAnalysis!();
        CompilerEigenform!();
    };
}

macro_rules! RustcEigenmatrix {
    () => {
        deps!();
        # [doc = " Rustc Eigenmatrix - Mathematical representation of the entire rustc compiler"] # [derive (Debug)] pub struct RustcEigenmatrix { # [doc = " Feature matrix of rustc components"] pub matrix : UrlMatrix , # [doc = " Rustc source analysis"] pub analysis : RustcAnalysis , # [doc = " Eigenform of the compiler"] pub eigenform : Option < CompilerEigenform > , }
    };
}

RustcEigenmatrix!();