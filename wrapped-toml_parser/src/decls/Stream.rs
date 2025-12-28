macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! Stream {
    () => {
        deps!();
        type Stream < 'i > = TokenSlice < 'i , Token > ;
    };
}

Stream!()