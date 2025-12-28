macro_rules! deps {
    () => {
        Error!();
        Prerelease!();
        ErrorKind!();
        Position!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl FromStr for Prerelease { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let (pre , rest) = prerelease_identifier (text) ? ; if ! rest . is_empty () { return Err (Error :: new (ErrorKind :: IllegalCharacter (Position :: Pre))) ; } Ok (pre) } }
    };
}

impl_64!()