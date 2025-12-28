macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for ExpnId { fn decode (s : & mut D) -> ExpnId { s . decode_expn_id () } }
    };
}

impl_59!()