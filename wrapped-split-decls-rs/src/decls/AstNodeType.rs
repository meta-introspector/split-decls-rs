// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "AstNodeType",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: [],
uses: ["Macro", "Type", "Struct", "All", "Expr", "Serialize", "Enum", "Impl", "Use", "Function", "Module", "Static", "Const", "AstNodeType", "Stmt", "Debug", "Deserialize", "Pat", "Clone", "Trait"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! AstNodeType {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum AstNodeType { Function , Struct , Enum , Impl , Trait , Module , Use , Const , Static , Type , Macro , Expr , Stmt , Pat , All , }
    };
}

AstNodeType!();