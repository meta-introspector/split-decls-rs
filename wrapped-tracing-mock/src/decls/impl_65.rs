macro_rules! deps {
    () => {
        SetActualSpanIdError!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl fmt :: Display for SetActualSpanIdError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Could not set `ExpecedId` to {new}, \
            it had already been set to {previous}" , new = self . new_span_id , previous = self . previous_span_id) } }
    };
}

impl_65!()