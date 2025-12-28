macro_rules! deps {
    () => {
        ParseLevelError!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl fmt :: Display for ParseLevelError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("error parsing level: expected one of \"error\", \"warn\", \
             \"info\", \"debug\", \"trace\", or a number 1-5" ,) } }
    };
}

impl_212!();