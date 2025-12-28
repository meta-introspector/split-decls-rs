macro_rules! deps {
    () => {
        Iter!();
        Error!();
        Release!();
        Result!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        pub fn parse (paren : Group , iter : Iter) -> Result < Release > { try_parse (iter) . map_err (| () | Error :: group (paren , "expected rustc release number, like 1.31")) }
    };
}

parse!();