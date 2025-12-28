macro_rules! deps {
    () => {
        ByteSymbol!();
        SpanDecoder!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for ByteSymbol { fn decode (s : & mut D) -> ByteSymbol { s . decode_byte_symbol () } }
    };
}

impl_282!()