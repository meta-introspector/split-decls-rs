macro_rules! deps {
    () => {
        Error!();
        Project!();
        Result!();
    };
}

macro_rules! build_all_tests {
    () => {
        deps!();
        pub (crate) fn build_all_tests (project : & Project) -> Result < Output > { let _ = cargo (project) . arg ("clean") . arg ("--package") . arg (& project . name) . arg ("--color=never") . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () ; cargo (project) . arg (if project . has_pass { "build" } else { "check" }) . args (target ()) . arg ("--bins") . args (features (project)) . arg ("--quiet") . arg ("--color=never") . arg ("--message-format=json") . arg ("--keep-going") . output () . map_err (Error :: Cargo) }
    };
}

build_all_tests!();