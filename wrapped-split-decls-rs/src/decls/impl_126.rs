// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_126",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["AstNodeType"],
uses: ["Function", "Type", "Module", "Macro", "Expr", "Const", "Static", "Stmt", "AstNodeType", "Pat", "Struct", "Enum", "Use", "Trait", "Impl", "All"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstNodeType!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl AstNodeType { fn as_str (& self) -> & 'static str { match self { AstNodeType :: Function => "Function" , AstNodeType :: Struct => "Struct" , AstNodeType :: Enum => "Enum" , AstNodeType :: Impl => "Impl" , AstNodeType :: Trait => "Trait" , AstNodeType :: Module => "Module" , AstNodeType :: Use => "Use" , AstNodeType :: Const => "Const" , AstNodeType :: Static => "Static" , AstNodeType :: Type => "Type" , AstNodeType :: Macro => "Macro" , AstNodeType :: Expr => "Expr" , AstNodeType :: Stmt => "Stmt" , AstNodeType :: Pat => "Pat" , AstNodeType :: All => "All" , } } }
    };
}

impl_126!();