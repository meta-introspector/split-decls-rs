use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn integration_test_unimacro_derive_build () -> Result < () > { let temp_dir = tempdir () . context ("Failed to create temporary directory") ? ; let temp_path = temp_dir . path () ; println ! ("Test workspace created at: {}" , temp_path . display ()) ; let test_crate_name = "unimacro_derive" ; let original_test_crate_path = PathBuf :: from ("../../unimacro_derive") ; let original_patch_build_rs_path = PathBuf :: from ("../patch-build-rs") ; let temp_test_crate_path = temp_path . join (test_crate_name) ; let temp_patch_build_rs_path = temp_path . join ("patch-build-rs") ; let temp_global_config_path = temp_path . join ("split-decls-rs.toml") ; let temp_workspace_cargo_toml = temp_path . join ("Cargo.toml") ; let workspace_cargo_toml_content = format ! (r#"
        [workspace]
        members = [
            "{}",
            "patch-build-rs/introspector_decl2_macros",
            "patch-build-rs/introspector_decl_core",
            "patch-build-rs/introspector_macro_helpers",
            "patch-build-rs/introspector_decl_common",
        ]

        [workspace.dependencies]
        proc-macro2 = {{ version = "1.0" }}
        quote = {{ version = "1.0" }}
        syn = {{ version = "2.0", features = ["full", "extra-traits", "visit", "fold", "visit-mut"] }}
        anyhow = {{ version = "1.0" }}
        toml = {{ version = "0.8" }}
        serde = {{ version = "1.0", features = ["derive"] }}

    "# , test_crate_name) ; fs :: write (& temp_workspace_cargo_toml , workspace_cargo_toml_content) . context ("Failed to write temporary workspace Cargo.toml") ? ; println ! ("Temporary workspace Cargo.toml created at: {}" , temp_workspace_cargo_toml . display ()) ; println ! ("Copying patch-build-rs from {} to {}" , original_patch_build_rs_path . display () , temp_patch_build_rs_path . display ()) ; copy_dir_recursive (& original_patch_build_rs_path , & temp_patch_build_rs_path) . context (format ! ("Failed to copy patch-build-rs from {} to {}" , original_patch_build_rs_path . display () , temp_patch_build_rs_path . display ())) ? ; println ! ("Copying test crate from {} to {}" , original_test_crate_path . display () , temp_test_crate_path . display ()) ; copy_dir_recursive (& original_test_crate_path , & temp_test_crate_path) . context (format ! ("Failed to copy test crate from {} to {}" , original_test_crate_path . display () , temp_test_crate_path . display ())) ? ; let dummy_config_content = format ! (r#"
        active_overlay_modules = []
        custom_prelude_overlay = "introspector_decl2_macros::prelude"
        crates_io_patches = {{}} # Added missing field
        [[patches."{}"]] # Corrected to array of tables
        path = "dummy_patch.rs" # Placeholder
    "# , test_crate_name) ; fs :: write (& temp_global_config_path , dummy_config_content) . context ("Failed to write dummy split-decls-rs.toml") ? ; println ! ("Dummy split-decls-rs.toml created at: {}" , temp_global_config_path . display ()) ; println ! ("Running split-decls-rs on the temporary workspace...") ; let global_config : SplitDeclsConfig = toml :: from_str (& fs :: read_to_string (& temp_global_config_path) ?) ? ; process_crates_in_path (& temp_path , & global_config , false , false ,) . context ("split-decls-rs execution failed") ? ; println ! ("split-decls-rs completed successfully on the test crate.") ; println ! ("Attempting to build the modified test crate from workspace root: {}" , temp_path . display ()) ; let output = Command :: new ("cargo") . arg ("check") . arg ("-p") . arg (test_crate_name) . current_dir (& temp_path) . output () . context ("Failed to execute cargo check on modified crate") ? ; if ! output . status . success () { eprintln ! ("Cargo check failed for {}:" , temp_test_crate_path . display ()) ; eprintln ! ("Stdout: {}" , String :: from_utf8_lossy (& output . stdout)) ; eprintln ! ("Stderr: {}" , String :: from_utf8_lossy (& output . stderr)) ; anyhow :: bail ! ("Cargo check failed for modified test crate.") ; } println ! ("Modified test crate compiled successfully: {}" , temp_test_crate_path . display ()) ; Ok (()) }