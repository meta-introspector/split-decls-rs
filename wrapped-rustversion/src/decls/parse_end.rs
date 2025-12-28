macro_rules! deps {
    () => {
        Error!();
        Iter!();
        Result!();
    };
}

macro_rules! parse_end {
    () => {
        deps!();
        pub fn parse_end (iter : Iter) -> Result < () > { match iter . next () { None => Ok (()) , Some (unexpected) => Err (Error :: new (unexpected . span () , "unexpected token")) , } }
    };
}

parse_end!();