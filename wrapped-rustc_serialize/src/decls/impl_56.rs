macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < D : Decoder , T : Decodable < D > > Decodable < D > for VecDeque < T > { fn decode (d : & mut D) -> VecDeque < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
    };
}

impl_56!()