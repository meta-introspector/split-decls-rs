// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "generate_build_rs_macros",
decl_type: "function",
source_file: "./src/buildrs_generator/static_parts.rs",
source_crate: ".",
deps: [],
uses: ["String", "GetToken", "ImplCallVisitor", "HashMap", "HashSet", "TokenStream", "Macro", "Some", "Visit", "Span"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! generate_build_rs_macros {
    () => {
        pub fn generate_build_rs_macros () -> TokenStream { quote ! { macro_rules ! GetToken { ($ token : tt) => { syn :: token ::$ token :: new (proc_macro2 :: Span :: call_site ()) } ; } macro_rules ! mkImplCallVisitor { (calls : $ init_calls : expr) => { struct ImplCallVisitor { calls : std :: collections :: HashMap < String , std :: collections :: HashSet < String >>, } impl ImplCallVisitor { fn new () -> Self { ImplCallVisitor { calls : $ init_calls } } } impl <'ast > syn :: visit :: Visit <'ast > for ImplCallVisitor { fn visit_macro (& mut self , i : &'ast syn :: Macro) { if let Some (path_segment) = i . path . segments . last () { let path_str = path_segment . ident . to_string () ; if path_str . ends_with ("_impl") { if let Some (module_ident) = i . path . segments . first () { let module_name = module_ident . ident . to_string () ; let fn_name = path_str ; self . calls . entry (module_name) . or_default () . insert (fn_name) ; } } } syn :: visit :: visit_macro (self , i) ; } } } ; } } }
    };
}

generate_build_rs_macros!();