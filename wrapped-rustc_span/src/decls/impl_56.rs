macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for Span { fn decode (s : & mut D) -> Span { s . decode_span () } }
    };
}

impl_56!()