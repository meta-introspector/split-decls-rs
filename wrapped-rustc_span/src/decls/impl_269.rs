macro_rules! deps {
    () => {
        Span!();
        SpanEncoder!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for Span { fn encode (& self , s : & mut E) { s . encode_span (* self) ; } }
    };
}

impl_269!()