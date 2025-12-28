macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        pub type Result < T , E = Error > = std :: result :: Result < T , E > ;
    };
}

Result!()