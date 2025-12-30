// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_259",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: ["AstVisitor"],
uses: ["PatStruct", "ItemTrait", "Generics", "ItemUse", "PatTuple", "Visit", "Block", "ExprPath", "ItemConst", "ExprUnary", "ItemStatic", "File", "ItemEnum", "Type", "ExprCall", "ItemStruct", "Path", "ItemFn", "Expr", "ExprIf", "TypePath", "TypeReference", "ItemImpl", "Ident", "ExprBinary", "Item", "Signature", "PatIdent", "AstVisitor", "ExprMatch", "Pat", "TypeTuple", "ItemMod", "ExprLit", "ExprBlock", "ExprMethodCall"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        AstVisitor!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < 'ast > Visit < 'ast > for AstVisitor < '_ > { impl_visitor_for_types ! (visit_file : File , visit_item : Item , visit_item_fn : ItemFn , visit_item_struct : ItemStruct , visit_item_enum : ItemEnum , visit_item_impl : ItemImpl , visit_item_trait : ItemTrait , visit_item_mod : ItemMod , visit_item_use : ItemUse , visit_item_const : ItemConst , visit_item_static : ItemStatic , visit_expr : Expr , visit_expr_call : ExprCall , visit_expr_method_call : ExprMethodCall , visit_expr_path : ExprPath , visit_expr_lit : ExprLit , visit_expr_block : ExprBlock , visit_expr_if : ExprIf , visit_expr_match : ExprMatch , visit_expr_binary : ExprBinary , visit_expr_unary : ExprUnary , visit_type : Type , visit_type_path : TypePath , visit_type_reference : TypeReference , visit_type_tuple : TypeTuple , visit_pat : Pat , visit_pat_ident : PatIdent , visit_pat_struct : PatStruct , visit_pat_tuple : PatTuple , visit_ident : Ident , visit_path : Path , visit_block : Block , visit_signature : Signature , visit_generics : Generics ,) ; }
    };
}

impl_259!();