macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! macro_444 {
    () => {
        deps!();
        index_impl ! (Span) ;
    };
}

macro_444!();