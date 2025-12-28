macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for Vec < T > { default fn decode (d : & mut D) -> Vec < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
    };
}

impl_26!();