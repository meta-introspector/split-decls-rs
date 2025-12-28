macro_rules! deps {
    () => {
        SpanDecoder!();
        SyntaxContext!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for SyntaxContext { fn decode (s : & mut D) -> SyntaxContext { s . decode_syntax_context () } }
    };
}

impl_284!()