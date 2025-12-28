macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for SyntaxContext { fn decode (s : & mut D) -> SyntaxContext { s . decode_syntax_context () } }
    };
}

impl_60!()