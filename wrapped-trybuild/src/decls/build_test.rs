macro_rules! deps {
    () => {
        Project!();
        Name!();
        Result!();
        Error!();
    };
}

macro_rules! build_test {
    () => {
        deps!();
        pub (crate) fn build_test (project : & Project , name : & Name) -> Result < Output > { let _ = cargo (project) . arg ("clean") . arg ("--package") . arg (& project . name) . arg ("--color=never") . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () ; cargo (project) . arg (if project . has_pass { "build" } else { "check" }) . args (target ()) . arg ("--bin") . arg (name) . args (features (project)) . arg ("--quiet") . arg ("--color=never") . arg ("--message-format=json") . output () . map_err (Error :: Cargo) }
    };
}

build_test!()