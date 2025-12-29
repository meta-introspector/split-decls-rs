// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "generate_build_rs_token_stream",
decl_type: "function",
source_file: "./src/buildrs_generator/mod.rs",
source_crate: ".",
deps: [],
uses: ["HashMap", "Item", "Path", "PatchSpec", "StringReplacement", "LitStr", "Generates", "Span", "SplitDeclsConfig", "Context", "TokenStream", "PathBuf", "VisitMut", "Result", "Visit", "Ok"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! generate_build_rs_token_stream {
    () => {
        # [doc = " Generates the TokenStream for the target build.rs file."] pub fn generate_build_rs_token_stream (decls_output_dir : & Path , crate_name_sanitized : & str ,) -> Result < TokenStream > { let decls_output_dir_lit = LitStr :: new (& decls_output_dir . display () . to_string () , Span :: call_site ()) ; let crate_name_sanitized_lit = LitStr :: new (crate_name_sanitized , Span :: call_site ()) ; let macros_ts = static_parts :: generate_build_rs_macros () ; let main_logic_ts = main_logic :: generate_main_logic_token_stream (& decls_output_dir_lit , & crate_name_sanitized_lit ,) ; let build_rs_token_stream = quote ! { use anyhow :: Context ; use anyhow :: Result ; use proc_macro2 :: { Span , TokenStream } ; use quote :: quote ; use syn :: LitStr ; use std :: fs ; use std :: path :: { Path , PathBuf } ; use std :: collections :: HashMap ; use syn :: { self , Item } ; use syn :: visit :: { self , Visit } ; use syn :: visit_mut :: { self , VisitMut } ; use split_decls_types :: { SplitDeclsConfig , PatchSpec , StringReplacement } ; use toml ; # macros_ts # main_logic_ts } ; Ok (build_rs_token_stream) }
    };
}

generate_build_rs_token_stream!();