macro_rules! deps {
    () => {
        GetBitsError!();
        FSETableError!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl core :: fmt :: Display for FSETableError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { FSETableError :: AccLogIsZero => write ! (f , "Acclog must be at least 1") , FSETableError :: AccLogTooBig { got , max } => { write ! (f , "Found FSE acc_log: {got} bigger than allowed maximum in this case: {max}") } FSETableError :: GetBitsError (e) => write ! (f , "{e:?}") , FSETableError :: ProbabilityCounterMismatch { got , expected_sum , symbol_probabilities , } => { write ! (f , "The counter ({got}) exceeded the expected sum: {expected_sum}. This means an error or corrupted data \n {symbol_probabilities:?}" ,) } FSETableError :: TooManySymbols { got } => { write ! (f , "There are too many symbols in this distribution: {got}. Max: 256" ,) } } } }
    };
}

impl_102!();