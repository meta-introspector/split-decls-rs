macro_rules! deps {
    () => {
        GetBitsError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl core :: fmt :: Display for GetBitsError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { GetBitsError :: TooManyBits { num_requested_bits , limit , } => { write ! (f , "Cant serve this request. The reader is limited to {limit} bits, requested {num_requested_bits} bits") } GetBitsError :: NotEnoughRemainingBits { requested , remaining , } => { write ! (f , "Can\'t read {requested} bits, only have {remaining} bits left") } } } }
    };
}

impl_8!();