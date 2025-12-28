macro_rules! deps {
    () => {
        Decoder!();
        Decodable!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < D : Decoder , K , V > Decodable < D > for BTreeMap < K , V > where K : Decodable < D > + PartialEq + Ord , V : Decodable < D > , { fn decode (d : & mut D) -> BTreeMap < K , V > { let len = d . read_usize () ; (0 .. len) . map (| _ | (Decodable :: decode (d) , Decodable :: decode (d))) . collect () } }
    };
}

impl_58!();