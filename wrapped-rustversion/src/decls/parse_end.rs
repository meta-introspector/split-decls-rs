macro_rules! deps {
    () => {
        Result!();
        Iter!();
        Error!();
    };
}

macro_rules! parse_end {
    () => {
        deps!();
        pub fn parse_end (iter : Iter) -> Result < () > { match iter . next () { None => Ok (()) , Some (unexpected) => Err (Error :: new (unexpected . span () , "unexpected token")) , } }
    };
}

parse_end!()