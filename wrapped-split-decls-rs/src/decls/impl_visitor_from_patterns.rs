// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_visitor_from_patterns",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: [],
uses: ["Visit"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! impl_visitor_from_patterns {
    () => {
        macro_rules ! impl_visitor_from_patterns { ($ visitor_struct : ident , [$ ($ pattern : ident) ,* $ (,) ?]) => { impl <'ast > syn :: visit :: Visit <'ast > for $ visitor_struct { gen_visitor_signatures ! ($ ($ pattern) ,*) ; } } ; }
    };
}

impl_visitor_from_patterns!();