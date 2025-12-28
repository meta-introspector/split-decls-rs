macro_rules! deps {
    () => {
        DatetimeParseError!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Display for DatetimeParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (what) = self . what { write ! (f , "invalid {what}") ? ; } else { "invalid datetime" . fmt (f) ? ; } if let Some (expected) = self . expected { write ! (f , ", expected {expected}") ? ; } Ok (()) } }
    };
}

impl_27!()