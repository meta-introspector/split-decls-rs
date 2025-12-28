macro_rules! deps {
    () => {
        DateTimeRangeError!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl fmt :: Display for DateTimeRangeError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { write ! (fmt , "a date could not be represented within the bounds the MS-DOS date range (1980-2107)") } }
    };
}

impl_142!();