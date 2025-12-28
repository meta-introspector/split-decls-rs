macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for Symbol { fn encode (& self , s : & mut E) { s . encode_symbol (* self) ; } }
    };
}

impl_46!()