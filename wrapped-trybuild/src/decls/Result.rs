macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        pub (crate) type Result < T > = std :: result :: Result < T , Error > ;
    };
}

Result!()