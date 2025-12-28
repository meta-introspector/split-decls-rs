macro_rules! deps {
    () => {
        BuildMetadata!();
        Position!();
        Error!();
        ErrorKind!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl FromStr for BuildMetadata { type Err = Error ; fn from_str (text : & str) -> Result < Self , Self :: Err > { let (build , rest) = build_identifier (text) ? ; if ! rest . is_empty () { return Err (Error :: new (ErrorKind :: IllegalCharacter (Position :: Build))) ; } Ok (build) } }
    };
}

impl_65!();