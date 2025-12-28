macro_rules! deps {
    () => {
        Release!();
        Result!();
        Error!();
        Iter!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        pub fn parse (paren : Group , iter : Iter) -> Result < Release > { try_parse (iter) . map_err (| () | Error :: group (paren , "expected rustc release number, like 1.31")) }
    };
}

parse!()