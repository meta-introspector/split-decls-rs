macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! Stream {
    () => {
        deps!();
        type Stream = std :: thread :: JoinHandle < Result < Vec < u8 > , std :: io :: Error > > ;
    };
}

Stream!()