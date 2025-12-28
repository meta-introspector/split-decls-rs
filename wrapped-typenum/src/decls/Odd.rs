macro_rules! deps {
    () => {
        B1!();
        UInt!();
    };
}

macro_rules! Odd {
    () => {
        deps!();
        # [doc = " The odd number 2*N + 1"] type Odd < N > = UInt < N , B1 > ;
    };
}

Odd!();