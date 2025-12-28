macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! Stream {
    () => {
        deps!();
        type Stream = std :: thread :: JoinHandle < Result < Vec < u8 > , std :: io :: Error > > ;
    };
}

Stream!();