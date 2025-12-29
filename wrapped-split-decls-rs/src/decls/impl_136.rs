// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_136",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["AstOperation", "SecurityContext"],
uses: ["AstOperation", "Strict", "SecurityContext", "Default"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        AstOperation!();
        SecurityContext!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl SecurityContext { pub fn check_permission (& self , operation : & AstOperation) -> bool { match self { SecurityContext :: Default => true , SecurityContext :: Strict (allowed) => { allowed . iter () . any (| op | { std :: mem :: discriminant (op) == std :: mem :: discriminant (operation) }) } } } }
    };
}

impl_136!();