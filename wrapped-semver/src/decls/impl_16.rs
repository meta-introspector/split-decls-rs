macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Display for Position { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (match self { Position :: Major => "major version number" , Position :: Minor => "minor version number" , Position :: Patch => "patch version number" , Position :: Pre => "pre-release identifier" , Position :: Build => "build metadata" , }) } }
    };
}

impl_16!()