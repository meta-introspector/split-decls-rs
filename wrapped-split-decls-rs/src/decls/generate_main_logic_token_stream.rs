// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "generate_main_logic_token_stream",
decl_type: "function",
source_file: "./src/buildrs_generator/main_logic.rs",
source_crate: ".",
deps: [],
uses: ["PathBuf", "Ok", "Result", "CARGO_MANIFEST_DIR", "LitStr", "TokenStream", "Some", "SplitDeclsConfig", "If", "Failed"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! generate_main_logic_token_stream {
    () => {
        pub fn generate_main_logic_token_stream (_decls_output_dir_lit : & LitStr , crate_name_sanitized_lit : & LitStr ,) -> TokenStream { quote ! { fn main () -> Result < () > { println ! ("cargo:rerun-if-changed=build.rs") ; println ! ("cargo:rerun-if-changed=.split-decls-config.toml") ; let config_path = PathBuf :: from (std :: env :: var ("CARGO_MANIFEST_DIR") ?) . join (".split-decls-config.toml") ; let config = SplitDeclsConfig :: load_from_file (& config_path) . context (format ! ("Failed to load config from {}" , config_path . display ())) ?; let current_crate_name_for_patch = # crate_name_sanitized_lit . to_string () ; if let Some (patches_for_crate) = config . patches . get (& current_crate_name_for_patch) { for patch_spec in patches_for_crate { let patch_path = PathBuf :: from (std :: env :: var ("CARGO_MANIFEST_DIR") ?) . join (& patch_spec . path) ; println ! ("cargo:rerun-if-changed={}" , patch_path . display ()) ; } } println ! ("cargo:note=build.rs finished. If `split-decls-rs` needs to be re-run, changes will be detected.") ; Ok (()) } } }
    };
}

generate_main_logic_token_stream!();