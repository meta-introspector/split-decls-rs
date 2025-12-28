macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < D : Decoder , T > Decodable < D > for BTreeSet < T > where T : Decodable < D > + PartialEq + Ord , { fn decode (d : & mut D) -> BTreeSet < T > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
    };
}

impl_60!()