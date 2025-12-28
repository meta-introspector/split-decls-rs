macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! BYTES_IN_LANE {
    () => {
        deps!();
        const BYTES_IN_LANE : usize = mem :: size_of :: < Bytes > () ;
    };
}

BYTES_IN_LANE!()