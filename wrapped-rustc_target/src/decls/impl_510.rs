macro_rules! deps {
    () => {
        SanitizerSet!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        # [doc = " Formats a sanitizer set as a comma separated list of sanitizers' names."] impl fmt :: Display for SanitizerSet { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut first = true ; for s in * self { let name = s . as_str () . unwrap_or_else (| | panic ! ("unrecognized sanitizer {s:?}")) ; if ! first { f . write_str (", ") ? ; } f . write_str (name) ? ; first = false ; } Ok (()) } }
    };
}

impl_510!()