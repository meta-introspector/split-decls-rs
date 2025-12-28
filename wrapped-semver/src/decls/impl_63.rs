macro_rules! deps {
    () => {
        ErrorKind!();
        Comparator!();
        Error!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl FromStr for Comparator { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let text = text . trim_start_matches (' ') ; let (comparator , pos , rest) = comparator (text) ? ; if ! rest . is_empty () { let unexpected = rest . chars () . next () . unwrap () ; return Err (Error :: new (ErrorKind :: UnexpectedCharAfter (pos , unexpected))) ; } Ok (comparator) } }
    };
}

impl_63!();