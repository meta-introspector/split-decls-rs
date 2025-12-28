macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (self . kind . as_str ()) ? ; if let Some (ref e) = self . err { e . fmt (f) ? ; } Ok (()) } }
    };
}

impl_424!();