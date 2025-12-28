macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Convenient wrapper around std::Result."] pub type Result < T > = :: std :: result :: Result < T , Error > ;
    };
}

Result!();