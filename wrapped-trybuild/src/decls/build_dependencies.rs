macro_rules! deps {
    () => {
        Project!();
        Error!();
        Result!();
    };
}

macro_rules! build_dependencies {
    () => {
        deps!();
        pub (crate) fn build_dependencies (project : & mut Project) -> Result < () > { match File :: open (path ! (project . workspace / "Cargo.lock")) { Ok (mut workspace_cargo_lock) => { if let Ok (mut new_cargo_lock) = File :: create (path ! (project . dir / "Cargo.lock")) { let _ = io :: copy (& mut workspace_cargo_lock , & mut new_cargo_lock) ; } } Err (err) => { if err . kind () == io :: ErrorKind :: NotFound { let _ = cargo (project) . arg ("generate-lockfile") . status () ; } } } let mut command = cargo (project) ; command . arg (if project . has_pass { "build" } else { "check" }) . args (target ()) . arg ("--bin") . arg (& project . name) . args (features (project)) ; let status = command . status () . map_err (Error :: Cargo) ? ; if ! status . success () { return Err (Error :: CargoFail) ; } project . keep_going = command . arg ("--keep-going") . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () . map (| status | status . success ()) . unwrap_or (false) ; Ok (()) }
    };
}

build_dependencies!()