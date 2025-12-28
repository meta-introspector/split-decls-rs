macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = "\nA generic streaming result.\n"] pub type Result < T = () , E = Error > = std :: result :: Result < T , E > ;
    };
}

Result!()