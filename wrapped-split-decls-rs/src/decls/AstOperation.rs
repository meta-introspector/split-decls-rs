// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstOperation",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: [],
uses: ["FileAccess", "NetworkAccess", "ParseItem", "String", "SystemCall", "Debug", "AST", "TransformItem", "AstOperation", "Clone", "GenerateCode"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! AstOperation {
    () => {
        # [doc = " AST operation types for security checking"] # [derive (Debug , Clone)] pub enum AstOperation { ParseItem , TransformItem , GenerateCode , FileAccess (String) , NetworkAccess , SystemCall (String) , }
    };
}

AstOperation!();