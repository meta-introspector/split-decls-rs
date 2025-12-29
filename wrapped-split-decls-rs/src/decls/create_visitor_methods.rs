// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "create_visitor_methods",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: [],
uses: ["Item", "Pat", "ExprCall", "Visit", "File", "Expr", "Ident", "ItemFn", "ItemStruct", "Type", "Block"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! create_visitor_methods {
    () => {
        macro_rules ! create_visitor_methods { ($ visitor_struct : ident , $ patterns : expr) => { impl <'ast > syn :: visit :: Visit <'ast > for $ visitor_struct <'_ > { fn visit_file (& mut self , node : &'ast syn :: File) { self . increment ("File") ; syn :: visit :: visit_file (self , node) ; } fn visit_item (& mut self , node : &'ast syn :: Item) { self . increment ("Item") ; syn :: visit :: visit_item (self , node) ; } fn visit_item_fn (& mut self , node : &'ast syn :: ItemFn) { self . increment ("ItemFn") ; syn :: visit :: visit_item_fn (self , node) ; } fn visit_item_struct (& mut self , node : &'ast syn :: ItemStruct) { self . increment ("ItemStruct") ; syn :: visit :: visit_item_struct (self , node) ; } fn visit_expr (& mut self , node : &'ast syn :: Expr) { self . increment ("Expr") ; syn :: visit :: visit_expr (self , node) ; } fn visit_expr_call (& mut self , node : &'ast syn :: ExprCall) { self . increment ("ExprCall") ; syn :: visit :: visit_expr_call (self , node) ; } fn visit_type (& mut self , node : &'ast syn :: Type) { self . increment ("Type") ; syn :: visit :: visit_type (self , node) ; } fn visit_pat (& mut self , node : &'ast syn :: Pat) { self . increment ("Pat") ; syn :: visit :: visit_pat (self , node) ; } fn visit_ident (& mut self , node : &'ast syn :: Ident) { self . increment ("Ident") ; syn :: visit :: visit_ident (self , node) ; } fn visit_block (& mut self , node : &'ast syn :: Block) { self . increment ("Block") ; syn :: visit :: visit_block (self , node) ; } } } ; }
    };
}

create_visitor_methods!();