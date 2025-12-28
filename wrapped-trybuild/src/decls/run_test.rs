macro_rules! deps {
    () => {
        Error!();
        Project!();
        Name!();
        Result!();
    };
}

macro_rules! run_test {
    () => {
        deps!();
        pub (crate) fn run_test (project : & Project , name : & Name) -> Result < Output > { cargo (project) . arg ("run") . args (target ()) . arg ("--bin") . arg (name) . args (features (project)) . arg ("--quiet") . arg ("--color=never") . output () . map_err (Error :: Cargo) }
    };
}

run_test!();