macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl fmt :: Display for Offset { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Z => write ! (f , "Z") , Self :: Custom { mut minutes } => { let mut sign = '+' ; if minutes < 0 { minutes *= - 1 ; sign = '-' ; } let hours = minutes / 60 ; let minutes = minutes % 60 ; write ! (f , "{sign}{hours:02}:{minutes:02}") } } } }
    };
}

impl_16!();