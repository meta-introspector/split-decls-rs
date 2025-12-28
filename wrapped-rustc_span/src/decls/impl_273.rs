macro_rules! deps {
    () => {
        SyntaxContext!();
        SpanEncoder!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for SyntaxContext { fn encode (& self , s : & mut E) { s . encode_syntax_context (* self) } }
    };
}

impl_273!()