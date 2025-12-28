macro_rules! deps {
    () => {
        Prerelease!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Prerelease { pub const EMPTY : Self = Prerelease { identifier : Identifier :: empty () , } ; pub fn new (text : & str) -> Result < Self , Error > { Prerelease :: from_str (text) } pub fn as_str (& self) -> & str { self . identifier . as_str () } pub fn is_empty (& self) -> bool { self . identifier . is_empty () } }
    };
}

impl_18!()