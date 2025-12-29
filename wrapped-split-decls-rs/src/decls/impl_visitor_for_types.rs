// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_visitor_for_types",
decl_type: "function",
source_file: "./src/ast_statistics.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! impl_visitor_for_types {
    () => {
        macro_rules ! impl_visitor_for_types { ($ ($ method : ident : $ type : ident) ,* $ (,) ?) => { $ (fn $ method (& mut self , node : &'ast syn ::$ type) { self . ast_stats . increment (stringify ! ($ type)) ; syn :: visit ::$ method (self , node) ; }) * } ; }
    };
}

impl_visitor_for_types!();