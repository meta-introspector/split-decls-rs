// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "gen_visitor_signatures",
decl_type: "function",
source_file: "./src/meta_pattern_visitor.rs",
source_crate: ".",
deps: [],
uses: [],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! gen_visitor_signatures {
    () => {
        macro_rules ! gen_visitor_signatures { ($ ($ pattern : expr) ,* $ (,) ?) => { $ (paste :: paste ! { fn [< visit_ $ pattern : lower >] (& mut self , node : &'ast syn ::$ pattern) { self . increment (stringify ! ($ pattern)) ; syn :: visit :: [< visit_ $ pattern : lower >] (self , node) ; } }) * } ; }
    };
}

gen_visitor_signatures!();