macro_rules! deps {
    () => {
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