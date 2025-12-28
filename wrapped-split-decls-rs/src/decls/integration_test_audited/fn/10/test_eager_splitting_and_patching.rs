use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_eager_splitting_and_patching () -> Result < () > { let temp_dir = tempdir () . context ("Failed to create temporary directory") ? ; let test_crate_path = temp_dir . path () . join ("my_test_crate") ; fs :: create_dir (& test_crate_path) ? ; fs :: create_dir (test_crate_path . join ("src")) ? ; let cargo_toml_content = r#"
        [package]
        name = "my-test-crate"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
        anyhow = "1.0"
        proc-macro2 = "1.0"
        quote = "1.0"
        syn = { version = "2.0", features = ["full", "extra-traits", "visit", "fold", "visit-mut"] }
        introspector_decl2_macros = { path = "../patch-build-rs/introspector_decl2_macros" }
    "# ; fs :: write (test_crate_path . join ("Cargo.toml") , cargo_toml_content) ? ; let lib_rs_content = r#"
        pub fn my_function(a: i32, b: i32) -> i32 {
            a + b
        }

        pub struct MyStruct {
            pub field: bool,
        }

        impl MyStruct {
            pub fn new() -> Self {
                MyStruct { field: true }
            }
        }
    "# ; fs :: write (test_crate_path . join ("src") . join ("lib.rs") , lib_rs_content) ? ; let patch_dir = temp_dir . path () . join ("patches") ; fs :: create_dir (& patch_dir) ? ; let my_patch_path = patch_dir . join ("my_patch.rs") ; let patch_content = r#"
        pub fn my_function(a: i32, b: i32) -> i32 {
            // Patched function
            a * b
        }
    "# ; fs :: write (& my_patch_path , patch_content) ? ; let patch_build_rs_dir = temp_dir . path () . join ("patch-build-rs") ; fs :: create_dir_all (patch_build_rs_dir . join ("introspector_decl2_macros")) ? ; fs :: write (patch_build_rs_dir . join ("introspector_decl2_macros/Cargo.toml") , r#"[package] name = "introspector_decl2_macros" version = "0.1.0" edition = "2021" [dependencies] proc-macro2 = "1.0" quote = "1.0" syn = { version = "2.0", features = ["full"] }"#) ? ; fs :: write (patch_build_rs_dir . join ("introspector_decl2_macros/src/lib.rs") , r#"pub mod prelude { pub use quote::quote; pub use syn::{parse_macro_input, Attribute, LitStr, Token}; } #[proc_macro] pub fn prelude(_input: proc_macro::TokenStream) -> proc_macro::TokenStream { proc_macro::TokenStream::new() } #[proc_macro_attribute] pub fn decl_MyTestCrate_decls_my_function(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream { item } "#) ? ; let global_config_content = format ! (r#"
        custom_prelude_overlay = "// My custom prelude for eager splitting"

        [patches."my-test-crate"]
        path = "{}"
        "# , my_patch_path . to_str () . context ("Path not UTF-8") ?) ; let global_config_path = temp_dir . path () . join ("split-decls-rs.toml") ; fs :: write (& global_config_path , global_config_content) ? ; let global_config : SplitDeclsConfig = toml :: from_str (& fs :: read_to_string (& global_config_path) ?) ? ; process_crate (& test_crate_path , & global_config , false) ? ; let decls_output_dir = test_crate_path . join ("src") . join ("decls") ; assert ! (decls_output_dir . exists ()) ; let my_function_decl_path = decls_output_dir . join ("my_test_crate_decls_my_function.rs") ; assert ! (my_function_decl_path . exists ()) ; let my_function_content = fs :: read_to_string (& my_function_decl_path) ? ; assert ! (my_function_content . contains ("// My custom prelude for eager splitting")) ; assert ! (my_function_content . contains ("a * b")) ; let my_struct_decl_path = decls_output_dir . join ("my_test_crate_decls_MyStruct.rs") ; assert ! (my_struct_decl_path . exists ()) ; let my_struct_content = fs :: read_to_string (& my_struct_decl_path) ? ; assert ! (my_struct_content . contains ("// My custom prelude for eager splitting")) ; assert ! (my_struct_content . contains ("pub struct MyStruct")) ; let my_struct_impl_decl_path = decls_output_dir . join ("my_test_crate_decls_impl_for_MyStruct.rs") ; assert ! (my_struct_impl_decl_path . exists ()) ; let my_struct_impl_content = fs :: read_to_string (& my_struct_impl_decl_path) ? ; assert ! (my_struct_impl_content . contains ("// My custom prelude for eager splitting")) ; assert ! (my_struct_impl_content . contains ("impl MyStruct")) ; let decl_invocation_path = decls_output_dir . join ("_decl_module_invocation.rs") ; assert ! (decl_invocation_path . exists ()) ; let decl_invocation_content = fs :: read_to_string (& decl_invocation_path) ? ; assert ! (decl_invocation_content . contains ("decl_module!(my_test_crate_decls_my_function, my_test_crate_decls_MyStruct, my_test_crate_decls_impl_for_MyStruct);")) ; let generated_build_rs_path = test_crate_path . join ("build.rs") ; assert ! (generated_build_rs_path . exists ()) ; let build_rs_content = fs :: read_to_string (& generated_build_rs_path) ? ; assert ! (build_rs_content . contains ("cargo:rerun-if-changed=build.rs")) ; assert ! (build_rs_content . contains ("cargo:rerun-if-changed=.split-decls-config.toml")) ; assert ! (build_rs_content . contains (& format ! ("cargo:rerun-if-changed={}" , my_patch_path . display ()))) ; assert ! (! build_rs_content . contains ("fs::create_dir_all")) ; assert ! (! build_rs_content . contains ("lib.rs content")) ; let output = audit_execute ! (Command :: new ("cargo") . arg ("check") . current_dir (& test_crate_path) . output ()) . context ("Failed to execute cargo check on eager-split crate") ? ; if ! output . status . success () { eprintln ! ("Cargo check failed for eager-split crate:") ; eprintln ! ("Stdout: {}" , String :: from_utf8_lossy (& output . stdout)) ; eprintln ! ("Stderr: {}" , String :: from_utf8_lossy (& output . stderr)) ; anyhow :: bail ! ("Cargo check failed for eager-split crate.") ; } Ok (()) }
}