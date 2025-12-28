macro_rules! deps {
    () => {
        Span!();
        SpanDecoder!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for Span { fn decode (s : & mut D) -> Span { s . decode_span () } }
    };
}

impl_280!();