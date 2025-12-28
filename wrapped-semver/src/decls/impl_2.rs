macro_rules! deps {
    () => {
        VersionReq!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Display for VersionReq { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { if self . comparators . is_empty () { return formatter . write_str ("*") ; } for (i , comparator) in self . comparators . iter () . enumerate () { if i > 0 { formatter . write_str (", ") ? ; } write ! (formatter , "{}" , comparator) ? ; } Ok (()) } }
    };
}

impl_2!();