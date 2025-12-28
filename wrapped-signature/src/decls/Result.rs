macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Result type."] # [doc = ""] # [doc = " A result with the `signature` crate's [`Error`] type."] pub type Result < T > = core :: result :: Result < T , Error > ;
    };
}

Result!();