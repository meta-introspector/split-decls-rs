macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " The result of a Syn parser."] pub type Result < T > = std :: result :: Result < T , Error > ;
    };
}

Result!()