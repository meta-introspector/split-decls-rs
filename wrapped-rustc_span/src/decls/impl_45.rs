macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for Span { fn encode (& self , s : & mut E) { s . encode_span (* self) ; } }
    };
}

impl_45!()