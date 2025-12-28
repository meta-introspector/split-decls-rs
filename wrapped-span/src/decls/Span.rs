macro_rules! deps {
    () => {
        SpanData!();
        SyntaxContext!();
    };
}

macro_rules! Span {
    () => {
        deps!();
        pub type Span = SpanData < SyntaxContext > ;
    };
}

Span!();