macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for ThinVec < T > { fn decode (d : & mut D) -> ThinVec < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
    };
}

impl_54!()