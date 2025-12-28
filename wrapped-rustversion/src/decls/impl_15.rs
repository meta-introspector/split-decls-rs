macro_rules! deps {
    () => {
        Date!();
        Result!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Display for Date { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "{:04}-{:02}-{:02}" , self . year , self . month , self . day ,) } }
    };
}

impl_15!();