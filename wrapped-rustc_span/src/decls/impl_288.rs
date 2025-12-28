macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for AttrId { fn decode (s : & mut D) -> AttrId { s . decode_attr_id () } }
    };
}

impl_288!();