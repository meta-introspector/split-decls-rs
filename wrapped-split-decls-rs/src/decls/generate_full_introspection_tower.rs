// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "generate_full_introspection_tower",
decl_type: "function",
source_file: "./src/introspect_macro.rs",
source_crate: ".",
deps: [],
uses: ["Bilinear", "Quaternionic", "Generate", "Linear", "Span", "Concrete", "TokenStream", "Ident", "Level"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! generate_full_introspection_tower {
    () => {
        # [doc = " Generate all introspection macros for the 8-level tower"] pub fn generate_full_introspection_tower () -> TokenStream { let mut output = TokenStream :: new () ; for level in 0 .. 8 { let level_name = format ! ("introspect_level_{}" , level) ; let level_ident = syn :: Ident :: new (& level_name , proc_macro2 :: Span :: call_site ()) ; let macro_impl = match level { 0 => quote ! { macro_rules ! # level_ident { ($ target : item) => { { println ! ("🎯 Level 0 (Concrete): {}" , stringify ! ($ target)) ; $ target } } ; } } , 1 => quote ! { macro_rules ! # level_ident { ($ target : item) => { { println ! ("📐 Level 1 (Linear): {}" , stringify ! ($ target)) ; $ target } } ; } } , 2 => quote ! { macro_rules ! # level_ident { ($ target : item) => { { println ! ("🔄 Level 2 (Bilinear): {}" , stringify ! ($ target)) ; $ target } } ; } } , 4 => quote ! { macro_rules ! # level_ident { ($ target : item) => { { println ! ("🌀 Level 4 (Quaternionic): {}" , stringify ! ($ target)) ; $ target } } ; } } , _ => quote ! { macro_rules ! # level_ident { ($ target : item) => { { println ! ("🔍 Level {}: {}" , # level , stringify ! ($ target)) ; $ target } } ; } } , } ; output . extend (macro_impl) ; } output }
    };
}

generate_full_introspection_tower!();