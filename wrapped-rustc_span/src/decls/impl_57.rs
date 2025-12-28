macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for Symbol { fn decode (s : & mut D) -> Symbol { s . decode_symbol () } }
    };
}

impl_57!()