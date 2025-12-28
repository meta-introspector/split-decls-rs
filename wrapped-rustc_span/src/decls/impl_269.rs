macro_rules! deps {
    () => {
        SpanEncoder!();
        Span!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for Span { fn encode (& self , s : & mut E) { s . encode_span (* self) ; } }
    };
}

impl_269!();