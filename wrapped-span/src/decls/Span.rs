macro_rules! deps {
    () => {
        SyntaxContext!();
        SpanData!();
    };
}

macro_rules! Span {
    () => {
        deps!();
        pub type Span = SpanData < SyntaxContext > ;
    };
}

Span!()