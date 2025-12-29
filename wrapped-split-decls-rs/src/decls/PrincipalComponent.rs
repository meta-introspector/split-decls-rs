// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "PrincipalComponent",
decl_type: "function",
source_file: "./src/rustc_eigenmatrix.rs",
source_crate: ".",
deps: [],
uses: ["String", "PrincipalComponent", "Vec", "Debug"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! PrincipalComponent {
    () => {
        # [derive (Debug)] pub struct PrincipalComponent { pub name : String , pub variance_explained : f64 , pub key_features : Vec < String > , }
    };
}

PrincipalComponent!();