macro_rules! deps {
    () => {
        CommaSeparated!();
        Result!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl < 'a > Display for CommaSeparated < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut first = true ; for & s in self . 0 { if ! first { f . write_str (", ") ? ; } f . write_str (s) ? ; first = false ; } Ok (()) } }
    };
}

impl_458!();