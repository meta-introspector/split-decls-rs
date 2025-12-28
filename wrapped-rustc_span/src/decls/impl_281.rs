macro_rules! deps {
    () => {
        SpanDecoder!();
        Symbol!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for Symbol { fn decode (s : & mut D) -> Symbol { s . decode_symbol () } }
    };
}

impl_281!()