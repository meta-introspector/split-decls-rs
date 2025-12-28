macro_rules! deps {
    () => {
        Date!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl fmt :: Display for Date { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:04}-{:02}-{:02}" , self . year , self . month , self . day) } }
    };
}

impl_14!()