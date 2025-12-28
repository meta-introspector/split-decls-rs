macro_rules! Result {
    () => {
        # [doc = "\nA generic streaming result.\n"] pub type Result < T = () , E = Error > = std :: result :: Result < T , E > ;
    };
}

Result!()