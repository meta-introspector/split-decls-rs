macro_rules! deps {
    () => {
        Version!();
        Comparator!();
        Error!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl Comparator { pub fn parse (text : & str) -> Result < Self , Error > { Comparator :: from_str (text) } pub fn matches (& self , version : & Version) -> bool { eval :: matches_comparator (self , version) } }
    };
}

impl_94!()