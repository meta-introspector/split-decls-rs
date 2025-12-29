// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "find_crates",
decl_type: "function",
source_file: "./src/crate_finder.rs",
source_crate: ".",
deps: ["CrateInfo"],
uses: ["Failed", "Result", "WalkDir", "CrateInfo", "Path", "Vec", "Ok", "Cargo.toml"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        CrateInfo!();
    };
}

macro_rules! find_crates {
    () => {
        deps!();
        pub fn find_crates (base_path : & Path) -> Result < Vec < CrateInfo > > { let mut discovered_crates = Vec :: new () ; for entry in walkdir :: WalkDir :: new (base_path) . into_iter () . filter_map (| e | e . ok ()) . filter (| e | e . file_name () == "Cargo.toml") { let cargo_toml_path = entry . path () ; let crate_path = cargo_toml_path . parent () . context (format ! ("Failed to get parent directory for Cargo.toml at {}" , cargo_toml_path . display ())) ? ; let lib_rs = crate_path . join ("src/lib.rs") ; if lib_rs . exists () { let crate_name = crate_path . file_name () . context (format ! ("Failed to get crate name for path {}" , crate_path . display ())) ? . to_string_lossy () . to_string () ; discovered_crates . push (CrateInfo { name : crate_name , original_path : crate_path . to_path_buf () , }) ; } } Ok (discovered_crates) }
    };
}

find_crates!();