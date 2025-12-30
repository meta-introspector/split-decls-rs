// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SecurityContext",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["AstOperation"],
uses: ["Strict", "ACL", "AstOperation", "Security", "Vec", "Clone", "SecurityContext", "AST", "Debug", "Default"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstOperation!();
    };
}

macro_rules! SecurityContext {
    () => {
        deps!();
        # [doc = " Security and ACL enum for AST operations  "] # [derive (Debug , Clone)] pub enum SecurityContext { Default , Strict (Vec < AstOperation >) , }
    };
}

SecurityContext!();