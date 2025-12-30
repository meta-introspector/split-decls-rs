// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstNodeType",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: [],
uses: ["Const", "Struct", "Macro", "Deserialize", "Function", "Type", "Serialize", "Clone", "Trait", "Debug", "Use", "Expr", "Enum", "Module", "Stmt", "Pat", "All", "Impl", "AstNodeType", "Static"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! AstNodeType {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum AstNodeType { Function , Struct , Enum , Impl , Trait , Module , Use , Const , Static , Type , Macro , Expr , Stmt , Pat , All , }
    };
}

AstNodeType!();