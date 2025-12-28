macro_rules! deps {
    () => {
        Error!();
        True!();
        Result!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'de > Visitor < 'de > for True { type Value = True ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("bool") } fn visit_bool < E > (self , b : bool) -> Result < Self :: Value , E > where E : de :: Error , { if b { Ok (True) } else { Err (de :: Error :: custom ("workspace=false is unsupported for package.edition" ,)) } } }
    };
}

impl_107!()