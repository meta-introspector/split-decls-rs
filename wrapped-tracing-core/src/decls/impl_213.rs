macro_rules! deps {
    () => {
        ParseLevelFilterError!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl fmt :: Display for ParseLevelFilterError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("error parsing level filter: expected one of \"off\", \"error\", \
            \"warn\", \"info\", \"debug\", \"trace\", or a number 0-5" ,) } }
    };
}

impl_213!();