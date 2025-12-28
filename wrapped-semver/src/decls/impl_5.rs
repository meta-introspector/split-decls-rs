macro_rules! deps {
    () => {
        BuildMetadata!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Display for BuildMetadata { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str (self . as_str ()) } }
    };
}

impl_5!()