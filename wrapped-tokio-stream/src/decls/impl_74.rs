macro_rules! deps {
    () => {
        StreamMap!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < K , V > Extend < (K , V) > for StreamMap < K , V > { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = (K , V) > , { self . entries . extend (iter) ; } }
    };
}

impl_74!()