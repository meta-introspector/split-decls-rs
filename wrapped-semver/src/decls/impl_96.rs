macro_rules! deps {
    () => {
        Identifier!();
        BuildMetadata!();
        Error!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl BuildMetadata { pub const EMPTY : Self = BuildMetadata { identifier : Identifier :: empty () , } ; pub fn new (text : & str) -> Result < Self , Error > { BuildMetadata :: from_str (text) } pub fn as_str (& self) -> & str { self . identifier . as_str () } pub fn is_empty (& self) -> bool { self . identifier . is_empty () } }
    };
}

impl_96!();