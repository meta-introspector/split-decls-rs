macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for SyntaxContext { fn encode (& self , s : & mut E) { s . encode_syntax_context (* self) } }
    };
}

impl_49!()