macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for ByteSymbol { fn decode (s : & mut D) -> ByteSymbol { s . decode_byte_symbol () } }
    };
}

impl_58!()