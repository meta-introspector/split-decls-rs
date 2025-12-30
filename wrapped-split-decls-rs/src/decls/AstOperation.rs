// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstOperation",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: [],
uses: ["NetworkAccess", "AstOperation", "TransformItem", "ParseItem", "AST", "Debug", "Clone", "FileAccess", "String", "GenerateCode", "SystemCall"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! AstOperation {
    () => {
        # [doc = " AST operation types for security checking"] # [derive (Debug , Clone)] pub enum AstOperation { ParseItem , TransformItem , GenerateCode , FileAccess (String) , NetworkAccess , SystemCall (String) , }
    };
}

AstOperation!();