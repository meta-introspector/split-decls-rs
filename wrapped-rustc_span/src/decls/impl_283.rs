macro_rules! deps {
    () => {
        ExpnId!();
        SpanDecoder!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for ExpnId { fn decode (s : & mut D) -> ExpnId { s . decode_expn_id () } }
    };
}

impl_283!()