// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "StrictSecurity",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["AstOperation"],
uses: ["Strict", "ACL", "StrictSecurity", "Vec", "AstOperation"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstOperation!();
    };
}

macro_rules! StrictSecurity {
    () => {
        deps!();
        # [doc = " Strict security implementation with ACL"] pub struct StrictSecurity { allowed_operations : Vec < AstOperation > , }
    };
}

StrictSecurity!();