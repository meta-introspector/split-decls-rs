// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_142",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["StrictSecurity", "AstOperation"],
uses: ["Vec", "StrictSecurity", "AstOperation"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        StrictSecurity!();
        AstOperation!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl StrictSecurity { pub fn new (allowed_operations : Vec < AstOperation >) -> Self { Self { allowed_operations } } }
    };
}

impl_142!();