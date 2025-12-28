macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! DUMMY_SP {
    () => {
        deps!();
        pub const DUMMY_SP : Span = Span ;
    };
}

DUMMY_SP!()