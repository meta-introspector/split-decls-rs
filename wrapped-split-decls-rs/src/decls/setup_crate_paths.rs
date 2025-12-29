// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "setup_crate_paths",
decl_type: "function",
source_file: "./src/paths.rs",
source_crate: ".",
deps: ["CratePaths"],
uses: ["Found", "Result", "Sets", "PATHS", "Total", "Ok", "Vec", "Crate", "CratePaths", "Cargo.toml", "CHECKING", "Path", "UTF-8"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        CratePaths!();
    };
}

macro_rules! setup_crate_paths {
    () => {
        deps!();
        # [doc = " Sets up and returns all relevant file paths for a given crate."] pub fn setup_crate_paths (crate_path : & Path) -> Result < CratePaths > { let crate_name_os_str = crate_path . file_name () . context ("Crate path has no file name") ? ; let crate_name = crate_name_os_str . to_str () . context ("Crate name is not valid UTF-8") ? ; let output_crate_path = crate_path . to_path_buf () ; let lib_rs_path = crate_path . join ("src") . join ("lib.rs") ; let main_rs_path = crate_path . join ("src") . join ("main.rs") ; println ! ("🔍 CHECKING PATHS for crate: {}" , crate_name) ; println ! ("   📚 lib.rs exists: {}" , lib_rs_path . exists ()) ; println ! ("   🎯 main.rs exists: {}" , main_rs_path . exists ()) ; let src_dir = crate_path . join ("src") ; let mut source_files = Vec :: new () ; if src_dir . exists () { for entry in std :: fs :: read_dir (& src_dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { println ! ("   📄 Found source file: {}" , path . display ()) ; source_files . push (path) ; } } } println ! ("   📊 Total source files found: {}" , source_files . len ()) ; let build_rs_path = crate_path . join ("build.rs") ; let cargo_toml_path = crate_path . join ("Cargo.toml") ; let decls_output_dir = output_crate_path . join ("src") . join ("decls") ; let target_config_path = output_crate_path . join (".split-decls-config.toml") ; Ok (CratePaths { crate_path : crate_path . to_path_buf () , crate_name : crate_name . to_string () , source_files , build_rs_path , cargo_toml_path , decls_output_dir , target_config_path , output_crate_path , }) }
    };
}

setup_crate_paths!();