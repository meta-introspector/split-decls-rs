use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_generate_wrapped_workspace");
# [test] fn test_generate_wrapped_workspace () -> Result < () > { let temp_dir = tempdir () . context ("Failed to create temporary directory") ? ; let original_project_root = temp_dir . path () . join ("original_project") ; fs :: create_dir (& original_project_root) ? ; let original_crate_a_path = original_project_root . join ("crate_a") ; fs :: create_dir_all (original_crate_a_path . join ("src")) ? ; fs :: write (original_crate_a_path . join ("Cargo.toml") , r#"
        [package]
        name = "crate-a"
        version = "0.1.0"
        edition = "2021"
        [dependencies]
        anyhow = "1.0"
        crate-b = { path = "../crate_b" }
    "#) ? ; fs :: write (original_crate_a_path . join ("src/lib.rs") , r#"
        pub fn a_function() -> &'static str { "Hello from A" }
    "#) ? ; let original_crate_b_path = original_project_root . join ("crate_b") ; fs :: create_dir_all (original_crate_b_path . join ("src")) ? ; fs :: write (original_crate_b_path . join ("Cargo.toml") , r#"
        [package]
        name = "crate-b"
        version = "0.1.0"
        edition = "2021"
        [dependencies]
        anyhow = "1.0"
    "#) ? ; fs :: write (original_crate_b_path . join ("src/lib.rs") , r#"
        pub fn b_function() -> &'static str { "Hello from B" }
    "#) ? ; let patch_build_rs_dir = temp_dir . path () . join ("patch-build-rs") ; fs :: create_dir_all (patch_build_rs_dir . join ("introspector_decl2_macros")) ? ; fs :: write (patch_build_rs_dir . join ("introspector_decl2_macros/Cargo.toml") , r#"[package] name = "introspector_decl2_macros" version = "0.1.0" edition = "2021" [dependencies] proc-macro2 = "1.0" quote = "1.0" syn = { version = "2.0", features = ["full"] }"#) ? ; fs :: write (patch_build_rs_dir . join ("introspector_decl2_macros/src/lib.rs") , r#"pub mod prelude { pub use quote::quote; pub use syn::{parse_macro_input, Attribute, LitStr, Token}; } #[proc_macro] pub fn prelude(_input: proc_macro::TokenStream) -> proc_macro::TokenStream { proc_macro::TokenStream::new() } #[proc_macro_attribute] pub fn decl_CrateA_decls_a_function(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream { item } #[proc_macro_attribute] pub fn decl_CrateB_decls_b_function(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream { item }"#) ? ; let global_config_content = r#"
        custom_prelude_overlay = "// Custom prelude for wrapped workspace"
    "# ; let global_config_path = original_project_root . join ("split-decls-rs.toml") ; fs :: write (& global_config_path , global_config_content) ? ; let global_config : SplitDeclsConfig = toml :: from_str (& fs :: read_to_string (& global_config_path) ?) ? ; let patch_config_content = format ! (r#"
        [[targets]]
        name = "crate-a"
        path = "{}"

        [[targets]]
        name = "crate-b"
        path = "{}"
    "# , original_crate_a_path . strip_prefix (original_project_root . parent () . unwrap ()) ?. display () , original_crate_b_path . strip_prefix (original_project_root . parent () . unwrap ()) ?. display () ,) ; let patch_config_path = original_project_root . join ("patch.toml") ; fs :: write (& patch_config_path , patch_config_content) ? ; let patch_config = split_decls_rs :: patch_config :: PatchConfig :: load_from_file (& patch_config_path) ? ; let wrapped_workspace_output_dir = temp_dir . path () . join ("wrapped-output") ; let current_crate_name = "split-decls-rs" ; generate_wrapped_workspace (& wrapped_workspace_output_dir , & patch_config , & global_config , current_crate_name , false , false , false ,) ? ; assert ! (wrapped_workspace_output_dir . exists ()) ; let wrapped_workspace_cargo_toml = wrapped_workspace_output_dir . join ("Cargo.toml") ; assert ! (wrapped_workspace_cargo_toml . exists ()) ; let ws_cargo_content = fs :: read_to_string (& wrapped_workspace_cargo_toml) ? ; assert ! (ws_cargo_content . contains (r#"members = ["wrapped-crate-a", "wrapped-crate-b"]"#)) ; let wrapped_crate_a_path = wrapped_workspace_output_dir . join ("wrapped-crate-a") ; assert ! (wrapped_crate_a_path . exists ()) ; let wrapped_crate_a_cargo_toml = wrapped_crate_a_path . join ("Cargo.toml") ; assert ! (wrapped_crate_a_cargo_toml . exists ()) ; let a_cargo_content = fs :: read_to_string (& wrapped_crate_a_cargo_toml) ? ; assert ! (a_cargo_content . contains (r#"name = "wrapped-crate-a""#)) ; assert ! (a_cargo_content . contains (& format ! (r#"crate-b = {{ path = "{}" }}"# , original_crate_b_path . canonicalize () ?. display ()))) ; let wrapped_crate_a_decls_dir = wrapped_crate_a_path . join ("src/decls") ; assert ! (wrapped_crate_a_decls_dir . exists ()) ; assert ! (wrapped_crate_a_decls_dir . join ("wrapped_crate_a_decls_a_function.rs") . exists ()) ; let a_func_content = fs :: read_to_string (wrapped_crate_a_decls_dir . join ("wrapped_crate_a_decls_a_function.rs")) ? ; assert ! (a_func_content . contains ("// Custom prelude for wrapped workspace")) ; let wrapped_crate_b_path = wrapped_workspace_output_dir . join ("wrapped-crate-b") ; assert ! (wrapped_crate_b_path . exists ()) ; let wrapped_crate_b_cargo_toml = wrapped_crate_b_path . join ("Cargo.toml") ; assert ! (wrapped_crate_b_cargo_toml . exists ()) ; let b_cargo_content = fs :: read_to_string (& wrapped_crate_b_cargo_toml) ? ; assert ! (b_cargo_content . contains (r#"name = "wrapped-crate-b""#)) ; let wrapped_crate_b_decls_dir = wrapped_crate_b_path . join ("src/decls") ; assert ! (wrapped_crate_b_decls_dir . exists ()) ; assert ! (wrapped_crate_b_decls_dir . join ("wrapped_crate_b_decls_b_function.rs") . exists ()) ; let b_func_content = fs :: read_to_string (wrapped_crate_b_decls_dir . join ("wrapped_crate_b_decls_b_function.rs")) ? ; assert ! (b_func_content . contains ("// Custom prelude for wrapped workspace")) ; let output = Command :: new ("cargo") . arg ("check") . current_dir (& wrapped_workspace_output_dir) . output () . context ("Failed to execute cargo check on wrapped workspace") ? ; if ! output . status . success () { eprintln ! ("Cargo check failed for wrapped workspace:") ; eprintln ! ("Stdout: {}" , String :: from_utf8_lossy (& output . stdout)) ; eprintln ! ("Stderr: {}" , String :: from_utf8_lossy (& output . stderr)) ; anyhow :: bail ! ("Cargo check failed for wrapped workspace.") ; } Ok (()) }
}