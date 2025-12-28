macro_rules! deps {
    () => {
        Decodable!();
        Decoder!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < D : Decoder , T , S > Decodable < D > for indexmap :: IndexSet < T , S > where T : Decodable < D > + Hash + Eq , S : BuildHasher + Default , { fn decode (d : & mut D) -> indexmap :: IndexSet < T , S > { let len = d . read_usize () ; (0 .. len) . map (| _ | Decodable :: decode (d)) . collect () } }
    };
}

impl_68!()