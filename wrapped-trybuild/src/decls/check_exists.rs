macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! check_exists {
    () => {
        deps!();
        fn check_exists (path : & Path) -> Result < () > { if path . exists () { return Ok (()) ; } match File :: open (path) { Ok (_) => Ok (()) , Err (err) => Err (Error :: Open (path . to_owned () , err)) , } }
    };
}

check_exists!()