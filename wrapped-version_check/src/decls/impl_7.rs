macro_rules! deps {
    () => {
        Date!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Display for Date { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let (y , m , d) = self . to_ymd () ; write ! (f , "{}-{:02}-{:02}" , y , m , d) } }
    };
}

impl_7!();