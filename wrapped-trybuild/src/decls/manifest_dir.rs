macro_rules! deps {
    () => {
        Result!();
        Directory!();
        Error!();
    };
}

macro_rules! manifest_dir {
    () => {
        deps!();
        pub (crate) fn manifest_dir () -> Result < Directory > { if let Some (manifest_dir) = env :: var_os ("CARGO_MANIFEST_DIR") { return Ok (Directory :: from (manifest_dir)) ; } let mut dir = Directory :: current () ? ; loop { if dir . join ("Cargo.toml") . exists () { return Ok (dir) ; } dir = dir . parent () . ok_or (Error :: ProjectDir) ? ; } }
    };
}

manifest_dir!();