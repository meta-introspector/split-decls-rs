macro_rules! deps {
    () => {
        Result!();
        Manifest!();
        Directory!();
        Error!();
    };
}

macro_rules! get_manifest {
    () => {
        deps!();
        pub (crate) fn get_manifest (manifest_dir : & Directory) -> Result < Manifest , Error > { let cargo_toml_path = manifest_dir . join ("Cargo.toml") ; let mut manifest = (| | { let manifest_str = fs :: read_to_string (& cargo_toml_path) ? ; let manifest : Manifest = toml :: from_str (& manifest_str) ? ; Ok (manifest) }) () . map_err (| err | Error :: GetManifest (cargo_toml_path , Box :: new (err))) ? ; fix_dependencies (& mut manifest . dependencies , manifest_dir) ; fix_dependencies (& mut manifest . dev_dependencies , manifest_dir) ; for target in manifest . target . values_mut () { fix_dependencies (& mut target . dependencies , manifest_dir) ; fix_dependencies (& mut target . dev_dependencies , manifest_dir) ; } Ok (manifest) }
    };
}

get_manifest!()