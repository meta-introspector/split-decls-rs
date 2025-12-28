macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < K : Ord + Hash , V > Extend < (K , V) > for Map < K , V > { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = (K , V) > , { self . map . extend (iter) ; } }
    };
}

impl_16!()