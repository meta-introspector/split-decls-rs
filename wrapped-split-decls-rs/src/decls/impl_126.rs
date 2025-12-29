// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_126",
decl_type: "function",
source_file: "./src/ast_reflector.rs",
source_crate: ".",
deps: ["AstNodeType"],
uses: ["Static", "Impl", "Struct", "Const", "Macro", "Expr", "Stmt", "Pat", "Trait", "All", "Enum", "Function", "AstNodeType", "Use", "Module", "Type"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
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