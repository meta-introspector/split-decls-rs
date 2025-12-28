macro_rules! deps {
    () => {
        SpanDecoder!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for CrateNum { fn decode (s : & mut D) -> CrateNum { s . decode_crate_num () } }
    };
}

impl_285!()