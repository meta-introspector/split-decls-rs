macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! PResult {
    () => {
        deps!();
        type PResult < T > = Result < T , ParseError > ;
    };
}

PResult!()