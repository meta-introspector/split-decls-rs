// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "StrictSecurity",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["AstOperation"],
uses: ["Strict", "AstOperation", "ACL", "StrictSecurity", "Vec"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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