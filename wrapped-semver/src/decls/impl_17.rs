macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("Error(\"") ? ; Display :: fmt (self , formatter) ? ; formatter . write_str ("\")") ? ; Ok (()) } }
    };
}

impl_17!()