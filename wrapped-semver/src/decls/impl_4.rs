macro_rules! deps {
    () => {
        Prerelease!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Display for Prerelease { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (self . as_str ()) } }
    };
}

impl_4!();