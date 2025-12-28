macro_rules! deps {
    () => {
        QuotedChar!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Display for QuotedChar { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { if self . 0 == '\0' { formatter . write_str ("'\\0'") } else { write ! (formatter , "{:?}" , self . 0) } } }
    };
}

impl_19!()